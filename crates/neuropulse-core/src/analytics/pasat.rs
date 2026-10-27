use crate::domain::models::{PasatSummary, PasatTrialPayload};

pub fn analyze_pasat_session(trials: &[PasatTrialPayload]) -> PasatSummary {
    let now = chrono::Local::now();
    let formatted_time = now.format("%d/%m/%Y às %H:%M:%S").to_string();

    if trials.is_empty() {
        return PasatSummary {
            total_trials: 0,
            total_correct: 0,
            accuracy_percentage: 0.0,
            mean_response_time_ms: 0.0,
            response_time_variability: 0.0,
            total_blinks: 0,
            mean_corrugator_tension: 0.0,
            mean_bpm: 0.0,
            max_cardiac_acceleration: 0.0,
            max_cardiac_deceleration: 0.0,
            mean_hrv_rmssd: 0.0,
            formatted_timestamp: formatted_time,
        };
    }

    let total = trials.len();
    let correct_count = trials.iter().filter(|t| t.is_correct).count();

    let valid_rts: Vec<f32> = trials
        .iter()
        .filter(|t| t.response_time_ms > 0.0)
        .map(|t| t.response_time_ms)
        .collect();

    let mean_rt = if !valid_rts.is_empty() {
        valid_rts.iter().sum::<f32>() / valid_rts.len() as f32
    } else {
        0.0
    };

    let variance = if valid_rts.len() > 1 {
        valid_rts
            .iter()
            .map(|rt| (rt - mean_rt).powi(2))
            .sum::<f32>()
            / (valid_rts.len() - 1) as f32
    } else {
        0.0
    };
    let std_dev_rt = variance.sqrt();

    let total_blinks = trials.iter().map(|t| t.blink_count_in_trial as usize).sum();
    let mean_corrugator = trials.iter().map(|t| t.corrugator_tension).sum::<f32>() / total as f32;

    PasatSummary {
        total_trials: total,
        total_correct: correct_count,
        accuracy_percentage: ((correct_count as f32 / total as f32) * 100.0 * 10.0).round() / 10.0,
        mean_response_time_ms: (mean_rt * 10.0).round() / 10.0,
        response_time_variability: (std_dev_rt * 10.0).round() / 10.0,
        total_blinks,
        mean_corrugator_tension: (mean_corrugator * 100.0).round() / 100.0f32,
        mean_bpm: 0.0,
        max_cardiac_acceleration: 0.0,
        max_cardiac_deceleration: 0.0,
        mean_hrv_rmssd: 0.0,
        formatted_timestamp: formatted_time,
    }
}
