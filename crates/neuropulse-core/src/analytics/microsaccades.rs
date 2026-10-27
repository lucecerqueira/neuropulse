use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MicrosaccadeAnalysis {
    pub microsaccade_count: usize,
    pub rate_per_second: f32,
    pub peak_velocity_deg_s: f32,
    pub instability_index: f32,
}

pub struct GazePoint {
    pub x: f64,
    pub y: f64,
    pub timestamp_ms: f64,
}

/// Algoritmo de Engbert & Kliegl para Detecção de Microssacadas baseada em Limite Elíptico de Velocidade
pub fn detect_microsaccades(points: &[GazePoint], lambda: f64) -> MicrosaccadeAnalysis {
    if points.len() < 5 {
        return MicrosaccadeAnalysis {
            microsaccade_count: 0,
            rate_per_second: 0.0,
            peak_velocity_deg_s: 0.0,
            instability_index: 0.0,
        };
    }

    // 1. Cálculo de Velocidades 2D via diferença centrada
    let mut vx: Vec<f64> = Vec::new();
    let mut vy: Vec<f64> = Vec::new();

    for i in 2..(points.len() - 2) {
        let dt = (points[i + 1].timestamp_ms - points[i - 1].timestamp_ms) / 1000.0;
        if dt > 0.0 {
            let v_x = (points[i + 1].x - points[i - 1].x) / (2.0 * dt);
            let v_y = (points[i + 1].y - points[i - 1].y) / (2.0 * dt);
            vx.push(v_x);
            vy.push(v_y);
        }
    }

    if vx.is_empty() {
        return MicrosaccadeAnalysis {
            microsaccade_count: 0,
            rate_per_second: 0.0,
            peak_velocity_deg_s: 0.0,
            instability_index: 0.0,
        };
    }

    // 2. Cálculo do Limite Elíptico de Velocidade (Median-based threshold)
    let med_vx = median(&vx);
    let med_vy = median(&vy);

    let eta_x = median_abs_deviation(&vx, med_vx);
    let eta_y = median_abs_deviation(&vy, med_vy);

    let threshold_x = lambda * eta_x;
    let threshold_y = lambda * eta_y;

    // 3. Identificação de eventos que violam a elipse de estabilidade foveal
    let mut count = 0;
    let mut peak_vel: f64 = 0.0;
    let mut in_microsaccade = false;

    for (v_x, v_y) in vx.iter().zip(vy.iter()) {
        let normalized_dist = ((v_x / threshold_x).powi(2) + (v_y / threshold_y).powi(2)).sqrt();
        let vel_mag = (v_x.powi(2) + v_y.powi(2)).sqrt();

        if normalized_dist > 1.0 {
            if !in_microsaccade {
                in_microsaccade = true;
                count += 1;
            }
            if vel_mag > peak_vel {
                peak_vel = vel_mag;
            }
        } else {
            in_microsaccade = false;
        }
    }

    let total_duration_s = (points.last().unwrap().timestamp_ms - points.first().unwrap().timestamp_ms) / 1000.0;
    let rate = if total_duration_s > 0.0 { count as f32 / total_duration_s as f32 } else { 0.0 };

    MicrosaccadeAnalysis {
        microsaccade_count: count,
        rate_per_second: (rate * 100.0).round() / 100.0f32,
        peak_velocity_deg_s: (peak_vel as f32 * 10.0).round() / 10.0,
        instability_index: ((count as f32 / (points.len() as f32 + 1.0)) * 100.0 * 10.0).round() / 10.0,
    }
}

fn median(v: &[f64]) -> f64 {
    let mut sorted = v.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

fn median_abs_deviation(v: &[f64], med: f64) -> f64 {
    let devs: Vec<f64> = v.iter().map(|val| (val - med).abs()).collect();
    median(&devs)
}
