use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

use neurosync_platform_rust::routes::app_router::create_router;

async fn setup_test_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL não configurada no ambiente");
    
    PgPool::connect(&db_url)
        .await
        .expect("Falha ao conectar no banco de dados de teste")
}

#[tokio::test]
async fn test_analyze_microsaccades_success() {
    let pool = setup_test_pool().await;
    let app = create_router(pool.clone());

    let session_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO eye_tracking_events (session_id, frame_number, gaze_x, gaze_y, gaze_z, ear_left, ear_right, confidence)
        VALUES 
            ($1, 1, 0.1, 0.2, 0.0, 0.0, 0.0, 1.0),
            ($1, 2, 0.2, 0.1, 0.0, 0.0, 0.0, 1.0),
            ($1, 3, 1.5, 2.0, 0.0, 0.0, 0.0, 1.0),
            ($1, 4, 0.1, 0.1, 0.0, 0.0, 0.0, 1.0),
            ($1, 5, 0.0, 0.1, 0.0, 0.0, 0.0, 1.0)
        "#,
        session_id
    )
    .execute(&pool)
    .await
    .expect("Falha ao inserir eventos de teste no banco");

    let payload = json!({
        "session_id": session_id,
        "heart_rate": {
            "bpm": 75.0,
            "acceleration_bpm_s": 0.5,
            "deceleration_bpm_s": -0.2,
            "hrv_rmssd_ms": 45.0
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/assessments/microsaccades/")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    sqlx::query!("DELETE FROM eye_tracking_events WHERE session_id = $1", session_id)
        .execute(&pool)
        .await
        .ok();
}
