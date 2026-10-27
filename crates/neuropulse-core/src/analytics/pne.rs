use ndarray::Array1;

pub fn calculate_pne(data: &Array1<f64>) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }

    let mean = data.mean().unwrap_or(0.0);
    let variance = data.mapv(|val| (val - mean).powi(2)).sum() / (data.len() - 1) as f64;

    if variance <= 0.0 {
        return 0.0;
    }

    let entropy = (1.0 + variance).ln();
    (entropy * 100.0).round() / 100.0
}
