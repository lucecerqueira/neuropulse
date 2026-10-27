use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use neuropulse_core::analytics::emotional_gonogo::analyze_gonogo_session;
use neuropulse_core::domain::models::{EmotionalGoNoGoPayload, EmotionalGoNoGoSummary, GenericResponse, HeartRateMetrics};

pub async fn process_gonogo_trial(
    State(pool): State<PgPool>,
    Json(payload): Json<EmotionalGoNoGoPayload>,
) -> Result<Json<GenericResponse>, (StatusCode, String)> {
    let hr = payload.heart_rate.unwrap_or(HeartRateMetrics {
        bpm: 0.0,
        acceleration_bpm_s: 0.0,
        deceleration_bpm_s: 0.0,
        hrv_rmssd_ms: 0.0,
    });

    sqlx::query!(
        r#"
        INSERT INTO emotional_gonogo_events (
            session_id, trial_number, stimulus_type, emotion_label,
            user_action, is_correct, reaction_time_ms, hold_time_ms,
            zygomaticus_activation, corrugator_activation, gaze_latency_ms,
            bpm, cardiac_acceleration, cardiac_deceleration, hrv_rmssd
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        "#,
        payload.session_id,
        payload.trial_number,
        payload.stimulus_type,
        payload.emotion_label,
        payload.user_action,
        payload.is_correct,
        payload.reaction_time_ms,
        payload.hold_time_ms,
        payload.zygomaticus_activation,
        payload.corrugator_activation,
        payload.gaze_latency_ms,
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
        message: "Ensaio Go/No-Go e rPPG registrados".to_string(),
    }))
}

pub async fn summarize_gonogo_session(
    State(pool): State<PgPool>,
    axum::extract::Path(session_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<EmotionalGoNoGoSummary>, (StatusCode, String)> {
    let rows = sqlx::query!(
        r#"
        SELECT trial_number, stimulus_type, emotion_label, user_action, is_correct,
               reaction_time_ms, hold_time_ms, zygomaticus_activation, corrugator_activation,
               gaze_latency_ms, bpm, cardiac_acceleration, cardiac_deceleration, hrv_rmssd
        FROM emotional_gonogo_events
        WHERE session_id = $1
        ORDER BY trial_number ASC
        "#,
        session_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let trials: Vec<EmotionalGoNoGoPayload> = rows
        .into_iter()
        .map(|r| EmotionalGoNoGoPayload {
            session_id,
            trial_number: r.trial_number,
            stimulus_type: r.stimulus_type,
            emotion_label: r.emotion_label,
            user_action: r.user_action,
            is_correct: r.is_correct,
            reaction_time_ms: r.reaction_time_ms,
            hold_time_ms: r.hold_time_ms,
            zygomaticus_activation: r.zygomaticus_activation,
            corrugator_activation: r.corrugator_activation,
            gaze_latency_ms: r.gaze_latency_ms,
            heart_rate: Some(HeartRateMetrics {
                bpm: r.bpm.unwrap_or(0.0),
                acceleration_bpm_s: r.cardiac_acceleration.unwrap_or(0.0),
                deceleration_bpm_s: r.cardiac_deceleration.unwrap_or(0.0),
                hrv_rmssd_ms: r.hrv_rmssd.unwrap_or(0.0),
            }),
        })
        .collect();

    let mut summary = analyze_gonogo_session(&trials);
    
    // Cálculo dos Biomarcadores Fisiológicos rPPG
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
