use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    jwt_secret: String,
}

#[derive(Deserialize)]
struct RegisterPayload {
    email: String,
    password: String,
    full_name: String,
}

#[derive(Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct SaveSessionPayload {
    level_completed: i32,
    avg_reaction_time_ms: f64,
    saccadic_latency_ms: Option<f64>,
    heart_rate_bpm: Option<f64>,
    respiration_rate_bpm: Option<f64>,
    hrv_rmssd: Option<f64>,
    cns_readiness_score: Option<f64>,
}

#[derive(Serialize, sqlx::FromRow)]
struct SessionRecord {
    id: uuid::Uuid,
    level_completed: Option<i32>,
    avg_reaction_time_ms: f64,
    saccadic_latency_ms: Option<f64>,
    heart_rate_bpm: Option<f64>,
    respiration_rate_bpm: Option<f64>,
    hrv_rmssd: Option<f64>,
    cns_readiness_score: Option<f64>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
    user_id: String,
    role: String,
    tier: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    tier: String,
    exp: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL nao configurada");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET nao configurada");

    println!("[INFO] Conectando ao PostgreSQL na porta 5433...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("[OK] Conexao estabelecida.");

    let state = AppState { pool, jwt_secret };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/sessions", post(save_session).get(get_user_sessions))
        .layer(cors)
        .with_state(state);

    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3001".to_string()).parse()?;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("[OK] Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "online",
        "service": "NeuroPulse API",
        "database": "connected"
    }))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let password_hash = payload.password;

    let user = sqlx::query!(
        r#"
        INSERT INTO users (email, password_hash, full_name, role, tier)
        VALUES ($1, $2, $3, 'client'::user_role, 'free_community'::subscription_tier)
        RETURNING id, role::text as "role!", tier::text as "tier!"
        "#,
        payload.email,
        password_hash,
        payload.full_name
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Erro ao cadastrar: {}", e)))?;

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("Timestamp invalido")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role.clone(),
        tier: user.tier.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
        role: user.role,
        tier: user.tier,
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query!(
        r#"
        SELECT id, password_hash, role::text as "role!", tier::text as "tier!"
        FROM users
        WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "Usuario nao encontrado".to_string()))?;

    if user.password_hash != payload.password {
        return Err((StatusCode::UNAUTHORIZED, "Senha incorreta".to_string()));
    }

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("Timestamp invalido")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role.clone(),
        tier: user.tier.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
        role: user.role,
        tier: user.tier,
    }))
}

async fn save_session(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SaveSessionPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Token ausente".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Formato de token invalido".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Token invalido".to_string()))?;

    let user_id = uuid::Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| (StatusCode::BAD_REQUEST, "ID de usuario invalido".to_string()))?;

    sqlx::query!(
        r#"
        INSERT INTO cognitive_sessions (
            user_id, level_completed, avg_reaction_time_ms, saccadic_latency_ms,
            heart_rate_bpm, respiration_rate_bpm, hrv_rmssd, cns_readiness_score
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        user_id,
        payload.level_completed,
        payload.avg_reaction_time_ms,
        payload.saccadic_latency_ms,
        payload.heart_rate_bpm,
        payload.respiration_rate_bpm,
        payload.hrv_rmssd,
        payload.cns_readiness_score
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao salvar sessao: {}", e)))?;

    Ok(Json(serde_json::json!({
        "status": "sucesso",
        "message": "Sessao gravada com sucesso no PostgreSQL"
    })))
}

async fn get_user_sessions(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<SessionRecord>>, (StatusCode, String)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Token ausente".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Formato de token invalido".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Token invalido".to_string()))?;

    let user_id = uuid::Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| (StatusCode::BAD_REQUEST, "ID de usuario invalido".to_string()))?;

    let sessions = sqlx::query_as!(
        SessionRecord,
        r#"
        SELECT id, level_completed, avg_reaction_time_ms, saccadic_latency_ms,
               heart_rate_bpm, respiration_rate_bpm, hrv_rmssd, cns_readiness_score, created_at
        FROM cognitive_sessions
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro ao buscar sessoes: {}", e)))?;

    Ok(Json(sessions))
}
