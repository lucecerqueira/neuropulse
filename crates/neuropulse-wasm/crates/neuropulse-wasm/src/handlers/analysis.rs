// Módulo de análise integrada de micro-sacadas e pletismografia rPPG
pub struct GazePoint {
    pub x: f64,
    pub y: f64,
    pub timestamp_ms: f64,
}

pub fn calculate_engine_threshold(velocities: &[f64], lambda: f64) -> f64 {
    if velocities.is_empty() {
        return 10.0;
    }
    let mut sorted_v = velocities.to_vec();
    sorted_v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = sorted_v[sorted_v.len() / 2];

    let mut dev: Vec<f64> = sorted_v.iter().map(|v| (v - median).abs()).collect();
    dev.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median_dev = dev[dev.len() / 2];

    let sigma = if median_dev > 0.0 { median_dev } else { 1.0 };
    (lambda * sigma).max(5.0)
}
