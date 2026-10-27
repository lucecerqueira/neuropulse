use ndarray::Array1;

pub fn calculate_bcea(x: &Array1<f64>, y: &Array1<f64>, k: f64) -> f64 {
    if x.len() < 2 || y.len() < 2 {
        return 0.0;
    }

    let mean_x = x.mean().unwrap_or(0.0);
    let mean_y = y.mean().unwrap_or(0.0);

    let var_x = x.mapv(|val| (val - mean_x).powi(2)).sum() / (x.len() - 1) as f64;
    let var_y = y.mapv(|val| (val - mean_y).powi(2)).sum() / (y.len() - 1) as f64;

    let cov_xy = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum::<f64>()
        / (x.len() - 1) as f64;

    let std_x = var_x.sqrt();
    let std_y = var_y.sqrt();

    if std_x == 0.0 || std_y == 0.0 {
        return 0.0;
    }

    let rho = cov_xy / (std_x * std_y);
    let bcea = 2.0 * std::f64::consts::PI * k * std_x * std_y * (1.0 - rho * rho).max(0.0).sqrt();

    (bcea * 100.0).round() / 100.0
}
