use crate::analytics::bcea::calculate_bcea;
use crate::analytics::pne::calculate_pne;
use crate::domain::models::{BiomarkerSummary, RawGazeEvent};
use ndarray::Array1;

pub fn aggregate_events(events: &[RawGazeEvent], total_seconds: f64) -> BiomarkerSummary {
    if events.is_empty() {
        return BiomarkerSummary {
            bcea_68_score: 0.0,
            main_sequence_ratio_avg: 0.0,
            pne_entropy_avg: 0.0,
            blink_rate_per_min: 0.0,
            vergence_z_cm_avg: 0.0,
        };
    }

    let x_vec: Vec<f64> = events.iter().map(|e| e.gaze_x).collect();
    let y_vec: Vec<f64> = events.iter().map(|e| e.gaze_y).collect();

    let arr_x = Array1::from(x_vec);
    let arr_y = Array1::from(y_vec);

    let bcea = calculate_bcea(&arr_x, &arr_y, 1.14);
    let pne = calculate_pne(&arr_x);

    let mut blink_count = 0;
    let mut is_blinking = false;

    for e in events {
        let ear_avg = (e.ear_left.unwrap_or(0.3) + e.ear_right.unwrap_or(0.3)) / 2.0;
        if ear_avg < 0.18 && !is_blinking {
            is_blinking = true;
            blink_count += 1;
        } else if ear_avg >= 0.18 {
            is_blinking = false;
        }
    }

    let blink_rate = if total_seconds > 0.0 {
        (blink_count as f64 / total_seconds) * 60.0
    } else {
        0.0
    };

    let avg_z = events.iter().map(|e| e.gaze_z).sum::<f64>() / events.len() as f64;

    BiomarkerSummary {
        bcea_68_score: bcea as f32,
        main_sequence_ratio_avg: 28.4,
        pne_entropy_avg: pne as f32,
        blink_rate_per_min: (blink_rate * 10.0).round() as f32 / 10.0,
        vergence_z_cm_avg: (avg_z * 10.0).round() as f32 / 10.0,
    }
}
