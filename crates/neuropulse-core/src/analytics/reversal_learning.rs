use crate::domain::models::{ReversalSummary, ReversalTrialPayload};

pub fn analyze_reversal_session(trials: &[ReversalTrialPayload]) -> ReversalSummary {
    let now = chrono::Local::now();
    let formatted_time = now.format("%d/%m/%Y às %H:%M:%S").to_string();

    if trials.is_empty() {
        return ReversalSummary {
            total_trials: 0,
            accuracy_percentage: 0.0,
            perseverative_errors: 0,
            trials_to_relearn: 0,
            mean_mouse_jitter: 0.0,
            post_error_corrugator_reactivity: 0.0,
            mean_bpm: 0.0,
            max_cardiac_acceleration: 0.0,
            max_cardiac_deceleration: 0.0,
            mean_hrv_rmssd: 0.0,
            formatted_timestamp: formatted_time,
        };
    }

    let total = trials.len();
    let correct_count = trials.iter().filter(|t| t.is_correct).count();

    let mut perseverative = 0;
    let mut post_reversal_trials = 0;
    let mut relearned = false;
    let mut consecutive_correct_after_reversal = 0;
    let mut trials_needed = 0;

    let mut jitter_sum = 0.0;
    let mut error_corrugator_sum = 0.0;
    let mut error_count = 0;

    for trial in trials {
        jitter_sum += trial.mouse_jitter_score;

        if !trial.is_correct {
            error_count += 1;
            error_corrugator_sum += trial.post_feedback_corrugator;
        }

        if trial.is_reversal_phase {
            post_reversal_trials += 1;
            
            if !trial.is_correct {
                perseverative += 1;
                consecutive_correct_after_reversal = 0;
            } else {
                consecutive_correct_after_reversal += 1;
                if consecutive_correct_after_reversal >= 3 && !relearned {
                    relearned = true;
                    trials_needed = post_reversal_trials;
                }
            }
        }
    }

    let mean_jitter = jitter_sum / total as f32;
    let mean_error_corrugator = if error_count > 0 {
        error_corrugator_sum / error_count as f32
    } else {
        0.0
    };

    ReversalSummary {
        total_trials: total,
        accuracy_percentage: ((correct_count as f32 / total as f32) * 100.0 * 10.0).round() / 10.0,
        perseverative_errors: perseverative,
        trials_to_relearn: if relearned { trials_needed } else { post_reversal_trials },
        mean_mouse_jitter: (mean_jitter * 100.0).round() / 100.0f32,
        post_error_corrugator_reactivity: (mean_error_corrugator * 100.0).round() / 100.0f32,
        mean_bpm: 0.0,
        max_cardiac_acceleration: 0.0,
        max_cardiac_deceleration: 0.0,
        mean_hrv_rmssd: 0.0,
        formatted_timestamp: formatted_time,
    }
}
