main_rs_content = """use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod app_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/neuropulse_db".to_string());

    println!("[INFO] Conectando ao PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("[OK] Conexao estabelecida.");

    let app = app_router::create_router(pool)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("[OK] Servidor Unificado rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
"""

with open("crates/neuropulse-server/src/main.rs", "w") as f:
    f.write(main_rs_content)

print("main.rs atualizado para carregar o app_router unificado.")
