use axum::{extract::State, http::StatusCode, Json};
use neuropulse_core::analytics::microsaccades::{detect_microsaccades as core_detect, GazePoint};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LocalGazePoint {
    pub x: f64,
    pub y: f64,
    pub timestamp_ms: f64,
}

#[derive(Deserialize)]
pub struct MicrosaccadePayload {
    pub points: Vec<LocalGazePoint>,
    pub lambda: Option<f64>,
}

pub async fn detect_microsaccades(
    State(_pool): State<PgPool>,
    Json(payload): Json<MicrosaccadePayload>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let lambda = payload.lambda.unwrap_or(6.0);
    let points: Vec<GazePoint> = payload
        .points
        .into_iter()
        .map(|p| GazePoint {
            x: p.x,
            y: p.y,
            timestamp_ms: p.timestamp_ms,
        })
        .collect();

    let result = core_detect(&points, lambda);
    Ok(Json(serde_json::to_value(result).unwrap_or_default()))
}
