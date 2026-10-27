use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use neuropulse_core::analytics::reversal_learning::analyze_reversal_session;
use neuropulse_core::domain::models::{GenericResponse, HeartRateMetrics, ReversalSummary, ReversalTrialPayload};

pub async fn process_reversal_trial(
    State(pool): State<PgPool>,
    Json(payload): Json<ReversalTrialPayload>,
) -> Result<Json<GenericResponse>, (StatusCode, String)> {
    let hr = payload.heart_rate.unwrap_or(HeartRateMetrics {
        bpm: 0.0,
        acceleration_bpm_s: 0.0,
        deceleration_bpm_s: 0.0,
        hrv_rmssd_ms: 0.0,
    });

    sqlx::query!(
        r#"
        INSERT INTO reversal_learning_events (
            session_id, trial_number, selected_option, is_correct,
            received_feedback, is_reversal_phase, response_time_ms,
            mouse_path_auc, mouse_jitter_score, post_feedback_corrugator,
            bpm, cardiac_acceleration, cardiac_deceleration, hrv_rmssd
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "#,
        payload.session_id,
        payload.trial_number,
        payload.selected_option,
        payload.is_correct,
        payload.received_feedback,
        payload.is_reversal_phase,
        payload.response_time_ms,
        payload.mouse_path_auc,
        payload.mouse_jitter_score,
        payload.post_feedback_corrugator,
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
        message: "Ensaio Reversal e rPPG registrados".to_string(),
    }))
}

pub async fn summarize_reversal_session(
    State(pool): State<PgPool>,
    axum::extract::Path(session_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<ReversalSummary>, (StatusCode, String)> {
    let rows = sqlx::query!(
        r#"
        SELECT trial_number, selected_option, is_correct, received_feedback,
               is_reversal_phase, response_time_ms, mouse_path_auc,
               mouse_jitter_score, post_feedback_corrugator,
               bpm, cardiac_acceleration, cardiac_deceleration, hrv_rmssd
        FROM reversal_learning_events
        WHERE session_id = $1
        ORDER BY trial_number ASC
        "#,
        session_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let trials: Vec<ReversalTrialPayload> = rows
        .into_iter()
        .map(|r| ReversalTrialPayload {
            session_id,
            trial_number: r.trial_number,
            selected_option: r.selected_option,
            is_correct: r.is_correct,
            received_feedback: r.received_feedback,
            is_reversal_phase: r.is_reversal_phase,
            response_time_ms: r.response_time_ms,
            mouse_path_auc: r.mouse_path_auc,
            mouse_jitter_score: r.mouse_jitter_score,
            post_feedback_corrugator: r.post_feedback_corrugator,
            heart_rate: Some(HeartRateMetrics {
                bpm: r.bpm.unwrap_or(0.0),
                acceleration_bpm_s: r.cardiac_acceleration.unwrap_or(0.0),
                deceleration_bpm_s: r.cardiac_deceleration.unwrap_or(0.0),
                hrv_rmssd_ms: r.hrv_rmssd.unwrap_or(0.0),
            }),
        })
        .collect();

    let mut summary = analyze_reversal_session(&trials);

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
