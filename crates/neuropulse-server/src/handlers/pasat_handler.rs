use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use neuropulse_core::analytics::pasat::analyze_pasat_session;
use neuropulse_core::domain::models::{GenericResponse, HeartRateMetrics, PasatSummary, PasatTrialPayload};

pub async fn process_pasat_trial(
    State(pool): State<PgPool>,
    Json(payload): Json<PasatTrialPayload>,
) -> Result<Json<GenericResponse>, (StatusCode, String)> {
    let hr = payload.heart_rate.unwrap_or(HeartRateMetrics {
        bpm: 0.0,
        acceleration_bpm_s: 0.0,
        deceleration_bpm_s: 0.0,
        hrv_rmssd_ms: 0.0,
    });

    sqlx::query!(
        r#"
        INSERT INTO pasat_events (
            session_id, trial_number, current_digit, previous_digit,
            expected_sum, user_sum, is_correct, response_time_ms,
            inter_digit_interval_ms, blink_count_in_trial, corrugator_tension,
            bpm, cardiac_acceleration, cardiac_deceleration, hrv_rmssd
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        "#,
        payload.session_id,
        payload.trial_number,
        payload.current_digit,
        payload.previous_digit,
        payload.expected_sum,
        payload.user_sum,
        payload.is_correct,
        payload.response_time_ms,
        payload.inter_digit_interval_ms,
        payload.blink_count_in_trial,
        payload.corrugator_tension,
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
        message: "Ensaio PASAT e rPPG registrados".to_string(),
    }))
}

pub async fn summarize_pasat_session(
    State(pool): State<PgPool>,
    axum::extract::Path(session_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<PasatSummary>, (StatusCode, String)> {
    let rows = sqlx::query!(
        r#"
        SELECT trial_number, current_digit, previous_digit, expected_sum, user_sum,
               is_correct, response_time_ms, inter_digit_interval_ms,
               blink_count_in_trial, corrugator_tension,
               bpm, cardiac_acceleration, cardiac_deceleration, hrv_rmssd
        FROM pasat_events
        WHERE session_id = $1
        ORDER BY trial_number ASC
        "#,
        session_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let trials: Vec<PasatTrialPayload> = rows
        .into_iter()
        .map(|r| PasatTrialPayload {
            session_id,
            trial_number: r.trial_number,
            current_digit: r.current_digit,
            previous_digit: r.previous_digit,
            expected_sum: r.expected_sum,
            user_sum: r.user_sum,
            is_correct: r.is_correct,
            response_time_ms: r.response_time_ms,
            inter_digit_interval_ms: r.inter_digit_interval_ms,
            blink_count_in_trial: r.blink_count_in_trial,
            corrugator_tension: r.corrugator_tension,
            heart_rate: Some(HeartRateMetrics {
                bpm: r.bpm.unwrap_or(0.0),
                acceleration_bpm_s: r.cardiac_acceleration.unwrap_or(0.0),
                deceleration_bpm_s: r.cardiac_deceleration.unwrap_or(0.0),
                hrv_rmssd_ms: r.hrv_rmssd.unwrap_or(0.0),
            }),
        })
        .collect();

    let mut summary = analyze_pasat_session(&trials);

    if !trials.is_empty() {
        let valid_bpm: Vec<f32> = trials.iter().filter_map(|t| t.heart_rate.as_ref().map(|h| h.bpm)).filter(|&b| b > 0.0).collect();
        if !valid_bpm.is_empty() {
            summary.mean_bpm = (valid_bpm.iter().sum::<f32>() / valid_bpm.len() as f32 * 10.0).round() / 10.0;
        }
        summary.max_cardiac_acceleration = trials.iter().filter_map(|t| t.heart_rate.as_ref().map(|h| h.acceleration_bpm_s)).fold(0.0, f32::max);
        summary.max_cardiac_deceleration = trials.iter().filter_map(|t| t.heart_rate.as_ref().map(|h| h.deceleration_bpm_s)).fold(0.0, f32::max);
        let valid_hrv: Vec<f32> = trials.iter().filter_map(|t| t.heart_rate.as_ref().map(|h| h.hrv_rmssd_ms)).filter(|&h| h > 0.0).collect();
        if !valid_hrv.is_empty() {
            summary.mean_hrv_rmssd = (valid_hrv.iter().sum::<f32>() / valid_hrv.len() as f32 * 10.0).round() / 10.0;
        }
    }

    Ok(Json(summary))
}
