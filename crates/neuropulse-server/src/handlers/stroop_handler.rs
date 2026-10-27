use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use neuropulse_core::domain::models::{GenericResponse, HeartRateMetrics, StroopSummary, StroopTrialPayload};

pub async fn process_stroop_trial(
    State(pool): State<PgPool>,
    Json(payload): Json<StroopTrialPayload>,
) -> Result<Json<GenericResponse>, (StatusCode, String)> {
    let hr = payload.heart_rate.unwrap_or(HeartRateMetrics {
        bpm: 0.0,
        acceleration_bpm_s: 0.0,
        deceleration_bpm_s: 0.0,
        hrv_rmssd_ms: 0.0,
    });

    sqlx::query!(
        r#"
        INSERT INTO stroop_events (
            session_id, trial_number, is_congruent, displayed_word,
            displayed_color, selected_color, is_correct, response_time_ms,
            bpm, cardiac_acceleration, cardiac_deceleration, hrv_rmssd
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        payload.session_id,
        payload.trial_number,
        payload.is_congruent,
        payload.displayed_word,
        payload.displayed_color,
        payload.selected_color,
        payload.is_correct,
        payload.response_time_ms,
        hr.bpm,
        hr.acceleration_bpm_s,
        hr.deceleration_bpm_s,
        hr.hrv_rmssd_ms
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(GenericResponse {
        status: "success".to_string(),
        message: "Ensaio Stroop e rPPG registrados".to_string(),
    }))
}

pub async fn summarize_stroop_session(
    State(pool): State<PgPool>,
    axum::extract::Path(session_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<StroopSummary>, (StatusCode, String)> {
    let now = chrono::Local::now();
    let formatted_time = now.format("%d/%m/%Y às %H:%M:%S").to_string();

    let rows = sqlx::query!(
        r#"
        SELECT trial_number, is_congruent, displayed_word, displayed_color,
               selected_color, is_correct, response_time_ms,
               bpm, cardiac_acceleration, cardiac_deceleration, hrv_rmssd
        FROM stroop_events
        WHERE session_id = $1
        ORDER BY trial_number ASC
        "#,
        session_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if rows.is_empty() {
        return Ok(Json(StroopSummary {
            total_trials: 0,
            accuracy_congruent: 0.0,
            accuracy_incongruent: 0.0,
            mean_rt_congruent_ms: 0.0,
            mean_rt_incongruent_ms: 0.0,
            stroop_effect_ms: 0.0,
            mean_bpm: 0.0,
            max_cardiac_acceleration: 0.0,
            max_cardiac_deceleration: 0.0,
            mean_hrv_rmssd: 0.0,
            formatted_timestamp: formatted_time,
        }));
    }

    let total = rows.len();
    let congruent_trials: Vec<_> = rows.iter().filter(|r| r.is_congruent).collect();
    let incongruent_trials: Vec<_> = rows.iter().filter(|r| !r.is_congruent).collect();

    let acc_cong = if !congruent_trials.is_empty() {
        (congruent_trials.iter().filter(|r| r.is_correct).count() as f32 / congruent_trials.len() as f32) * 100.0
    } else { 0.0 };

    let acc_incong = if !incongruent_trials.is_empty() {
        (incongruent_trials.iter().filter(|r| r.is_correct).count() as f32 / incongruent_trials.len() as f32) * 100.0
    } else { 0.0 };

    let rt_cong = if !congruent_trials.is_empty() {
        congruent_trials.iter().map(|r| r.response_time_ms).sum::<f32>() / congruent_trials.len() as f32
    } else { 0.0 };

    let rt_incong = if !incongruent_trials.is_empty() {
        incongruent_trials.iter().map(|r| r.response_time_ms).sum::<f32>() / incongruent_trials.len() as f32
    } else { 0.0 };

    let valid_bpm: Vec<f32> = rows.iter().map(|r| r.bpm.unwrap_or(0.0)).filter(|&b| b > 0.0).collect();
    let mean_bpm = if !valid_bpm.is_empty() { valid_bpm.iter().sum::<f32>() / valid_bpm.len() as f32 } else { 0.0 };

    let max_acc = rows.iter().map(|r| r.cardiac_acceleration.unwrap_or(0.0)).fold(0.0, f32::max);
    let max_dec = rows.iter().map(|r| r.cardiac_deceleration.unwrap_or(0.0)).fold(0.0, f32::max);

    let valid_hrv: Vec<f32> = rows.iter().map(|r| r.hrv_rmssd.unwrap_or(0.0)).filter(|&h| h > 0.0).collect();
    let mean_hrv = if !valid_hrv.is_empty() { valid_hrv.iter().sum::<f32>() / valid_hrv.len() as f32 } else { 0.0 };

    Ok(Json(StroopSummary {
        total_trials: total,
        accuracy_congruent: (acc_cong * 10.0).round() / 10.0,
        accuracy_incongruent: (acc_incong * 10.0).round() / 10.0,
        mean_rt_congruent_ms: (rt_cong * 10.0).round() / 10.0,
        mean_rt_incongruent_ms: (rt_incong * 10.0).round() / 10.0,
        stroop_effect_ms: ((rt_incong - rt_cong) * 10.0).round() / 10.0,
        mean_bpm: (mean_bpm * 10.0).round() / 10.0,
        max_cardiac_acceleration: (max_acc * 100.0).round() / 100.0,
        max_cardiac_deceleration: (max_dec * 100.0).round() / 100.0,
        mean_hrv_rmssd: (mean_hrv * 10.0).round() / 10.0,
        formatted_timestamp: formatted_time,
    }))
}
