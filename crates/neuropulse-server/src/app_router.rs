use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::finalize::finalize_session;
use crate::health::health_check;
use crate::ingest::ingest_events;
use crate::handlers::{
    gonogo_handler::{process_gonogo_trial, summarize_gonogo_session},
    microsaccades_handler::detect_microsaccades,
    pasat_handler::{process_pasat_trial, summarize_pasat_session},
    reversal_handler::{process_reversal_trial, summarize_reversal_session},
    stroop_handler::{process_stroop_trial, summarize_stroop_session},
};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/ingest", post(ingest_events))
        .route("/api/v1/finalize", post(finalize_session))
        .route("/api/v1/tests/gonogo/trial", post(process_gonogo_trial))
        .route("/api/v1/tests/gonogo/summary", post(summarize_gonogo_session))
        .route("/api/v1/tests/stroop/trial", post(process_stroop_trial))
        .route("/api/v1/tests/stroop/summary", post(summarize_stroop_session))
        .route("/api/v1/tests/pasat/trial", post(process_pasat_trial))
        .route("/api/v1/tests/pasat/summary", post(summarize_pasat_session))
        .route("/api/v1/tests/reversal/trial", post(process_reversal_trial))
        .route("/api/v1/tests/reversal/summary", post(summarize_reversal_session))
        .route("/api/v1/analytics/microsaccades", post(detect_microsaccades))
        .with_state(pool)
}
