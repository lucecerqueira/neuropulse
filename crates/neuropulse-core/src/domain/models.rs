use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==========================================
// MÓDULO BASE DE RASTREAMENTO E INGESTÃO 3D
// ==========================================

#[derive(Debug, Deserialize)]
pub struct GazeEventPayload {
    pub time: DateTime<Utc>,
    pub frame_number: i32,
    pub gaze_x: f32,
    pub gaze_y: f32,
    pub confidence: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct BatchIngestRequest {
    pub session_id: Uuid,
    pub events: Vec<GazeEventPayload>,
}

#[derive(Debug, Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomarkerSummary {
    pub bcea_68_score: f32,
    pub main_sequence_ratio_avg: f32,
    pub pne_entropy_avg: f32,
    pub blink_rate_per_min: f32,
    pub vergence_z_cm_avg: f32,
}

pub struct RawGazeEvent {
    pub gaze_x: f64,
    pub gaze_y: f64,
    pub gaze_z: f64,
    pub ear_left: Option<f64>,
    pub ear_right: Option<f64>,
}

// ==========================================
// MÓDULO FISIOLÓGICO: rPPG (HEART RATE METRICS)
// ==========================================

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeartRateMetrics {
    pub bpm: f32,                 // Batimentos por minuto instantâneos
    pub acceleration_bpm_s: f32,  // Aceleração cardíaca (dBPM / dt)
    pub deceleration_bpm_s: f32,  // Desaceleração cardíaca
    pub hrv_rmssd_ms: f32,        // Variabilidade da frequência cardíaca (Root Mean Square)
}

// ==========================================
// MÓDULO 1: EMOTIONAL GO/NO-GO PARADIGM
// ==========================================

#[derive(Debug, Deserialize)]
pub struct EmotionalGoNoGoPayload {
    pub session_id: Uuid,
    pub trial_number: i32,
    pub stimulus_type: String, // "NEUTRAL" (Go) ou "EMOTIONAL" (No-Go)
    pub emotion_label: String, // "HAPPY", "ANGRY", "NEUTRAL"
    pub user_action: String,   // "KEY_PRESS", "MOUSE_CLICK", "TIMEOUT"
    pub is_correct: bool,
    pub reaction_time_ms: f32,
    pub hold_time_ms: f32,
    pub zygomaticus_activation: f32, // AU12 (Alegria)
    pub corrugator_activation: f32,  // AU04 (Raiva/Tensão)
    pub gaze_latency_ms: f32,
    pub heart_rate: Option<HeartRateMetrics>, // Integrado rPPG
}

#[derive(Debug, Serialize)]
pub struct EmotionalGoNoGoSummary {
    pub total_trials: usize,
    pub accuracy_percentage: f32,
    pub mean_go_rt_ms: f32,
    pub commission_errors_nogo: usize,
    pub omission_errors_go: usize,
    pub mean_zygomaticus_reactivity: f32,
    pub mean_corrugator_reactivity: f32,
    pub mean_bpm: f32,
    pub max_cardiac_acceleration: f32,
    pub max_cardiac_deceleration: f32,
    pub mean_hrv_rmssd: f32,
    pub formatted_timestamp: String,
}

// ==========================================
// MÓDULO 2: PASAT DIGITAL (CARGA COGNITIVA)
// ==========================================

#[derive(Debug, Deserialize)]
pub struct PasatTrialPayload {
    pub session_id: Uuid,
    pub trial_number: i32,
    pub current_digit: i32,
    pub previous_digit: i32,
    pub expected_sum: i32,
    pub user_sum: Option<i32>,
    pub is_correct: bool,
    pub response_time_ms: f32,
    pub inter_digit_interval_ms: i32,
    pub blink_count_in_trial: i32,
    pub corrugator_tension: f32,
    pub heart_rate: Option<HeartRateMetrics>, // Integrado rPPG
}

#[derive(Debug, Serialize)]
pub struct PasatSummary {
    pub total_trials: usize,
    pub total_correct: usize,
    pub accuracy_percentage: f32,
    pub mean_response_time_ms: f32,
    pub response_time_variability: f32,
    pub total_blinks: usize,
    pub mean_corrugator_tension: f32,
    pub mean_bpm: f32,
    pub max_cardiac_acceleration: f32,
    pub max_cardiac_deceleration: f32,
    pub mean_hrv_rmssd: f32,
    pub formatted_timestamp: String,
}

// ==========================================
// MÓDULO 3: REVERSAL LEARNING (PLASTICIDADE)
// ==========================================

#[derive(Debug, Deserialize)]
pub struct ReversalTrialPayload {
    pub session_id: Uuid,
    pub trial_number: i32,
    pub selected_option: String,
    pub is_correct: bool,
    pub received_feedback: String,
    pub is_reversal_phase: bool,
    pub response_time_ms: f32,
    pub mouse_path_auc: f32,
    pub mouse_jitter_score: f32,
    pub post_feedback_corrugator: f32,
    pub heart_rate: Option<HeartRateMetrics>, // Integrado rPPG
}

#[derive(Debug, Serialize)]
pub struct ReversalSummary {
    pub total_trials: usize,
    pub accuracy_percentage: f32,
    pub perseverative_errors: usize,
    pub trials_to_relearn: usize,
    pub mean_mouse_jitter: f32,
    pub post_error_corrugator_reactivity: f32,
    pub mean_bpm: f32,
    pub max_cardiac_acceleration: f32,
    pub max_cardiac_deceleration: f32,
    pub mean_hrv_rmssd: f32,
    pub formatted_timestamp: String,
}

// ==========================================
// MÓDULO 4: TESTE DE STROOP
// ==========================================

#[derive(Debug, Deserialize)]
pub struct StroopTrialPayload {
    pub session_id: Uuid,
    pub trial_number: i32,
    pub is_congruent: bool,
    pub displayed_word: String,
    pub displayed_color: String,
    pub selected_color: String,
    pub is_correct: bool,
    pub response_time_ms: f32,
    pub heart_rate: Option<HeartRateMetrics>, // Integrado rPPG
}

#[derive(Debug, Serialize)]
pub struct StroopSummary {
    pub total_trials: usize,
    pub accuracy_congruent: f32,
    pub accuracy_incongruent: f32,
    pub mean_rt_congruent_ms: f32,
    pub mean_rt_incongruent_ms: f32,
    pub stroop_effect_ms: f32, // RT Incongruente - RT Congruente
    pub mean_bpm: f32,
    pub max_cardiac_acceleration: f32,
    pub max_cardiac_deceleration: f32,
    pub mean_hrv_rmssd: f32,
    pub formatted_timestamp: String,
}

// ==========================================
// MÓDULO 5: GESTÃO E CADASTRO DE PACIENTES
// ==========================================

#[derive(Debug, Deserialize)]
pub struct CreatePatientPayload {
    pub full_name: String,
    pub medical_record_id: String,
    pub birth_date: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Patient {
    pub id: Uuid,
    pub full_name: String,
    pub medical_record_id: String,
    pub birth_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}
