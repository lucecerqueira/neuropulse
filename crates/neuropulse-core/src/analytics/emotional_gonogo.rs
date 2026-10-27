use crate::domain::models::{EmotionalGoNoGoPayload, EmotionalGoNoGoSummary};

pub fn analyze_gonogo_session(trials: &[EmotionalGoNoGoPayload]) -> EmotionalGoNoGoSummary {
    let now = chrono::Local::now();
    let formatted_time = now.format("%d/%m/%Y às %H:%M:%S").to_string();

    if trials.is_empty() {
        return EmotionalGoNoGoSummary {
            total_trials: 0,
            accuracy_percentage: 0.0,
            mean_go_rt_ms: 0.0,
            commission_errors_nogo: 0,
            omission_errors_go: 0,
            mean_zygomaticus_reactivity: 0.0,
            mean_corrugator_reactivity: 0.0,
            mean_bpm: 0.0,
            max_cardiac_acceleration: 0.0,
            max_cardiac_deceleration: 0.0,
            mean_hrv_rmssd: 0.0,
            formatted_timestamp: formatted_time,
        };
    }

    let total = trials.len();
    let correct_count = trials.iter().filter(|t| t.is_correct).count();

    let go_trials: Vec<&EmotionalGoNoGoPayload> = trials
        .iter()
        .filter(|t| t.stimulus_type == "NEUTRAL")
        .collect();

    let nogo_trials: Vec<&EmotionalGoNoGoPayload> = trials
        .iter()
        .filter(|t| t.stimulus_type == "EMOTIONAL")
        .collect();

    // Erros de comissão: acionou quando era No-Go
    let commission_errors = nogo_trials
        .iter()
        .filter(|t| t.user_action != "TIMEOUT")
        .count();

    // Erros de omissão: não acionou quando era Go
    let omission_errors = go_trials
        .iter()
        .filter(|t| t.user_action == "TIMEOUT")
        .count();

    // Tempo médio de reação nos ensaios Go corretos
    let valid_go_rts: Vec<f32> = go_trials
        .iter()
        .filter(|t| t.is_correct && t.reaction_time_ms > 0.0)
        .map(|t| t.reaction_time_ms)
        .collect();

    let mean_rt = if !valid_go_rts.is_empty() {
        valid_go_rts.iter().sum::<f32>() / valid_go_rts.len() as f32
    } else {
        0.0
    };

    let mean_zyg = trials.iter().map(|t| t.zygomaticus_activation).sum::<f32>() / total as f32;
    let mean_corr = trials.iter().map(|t| t.corrugator_activation).sum::<f32>() / total as f32;

    EmotionalGoNoGoSummary {
        total_trials: total,
        accuracy_percentage: ((correct_count as f32 / total as f32) * 100.0 * 10.0).round() / 10.0,
        mean_go_rt_ms: (mean_rt * 10.0).round() / 10.0,
        commission_errors_nogo: commission_errors,
        omission_errors_go: omission_errors,
        mean_zygomaticus_reactivity: (mean_zyg * 100.0).round() / 100.0f32,
        mean_corrugator_reactivity: (mean_corr * 100.0).round() / 100.0f32,
        mean_bpm: 0.0,
        max_cardiac_acceleration: 0.0,
        max_cardiac_deceleration: 0.0,
        mean_hrv_rmssd: 0.0,
        formatted_timestamp: formatted_time,
    }
}
