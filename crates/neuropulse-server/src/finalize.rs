use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

pub async fn finalize_session(
    State(_pool): State<PgPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(payload))
}
