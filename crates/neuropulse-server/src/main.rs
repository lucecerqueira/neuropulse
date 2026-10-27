pub mod app_router;
pub mod handlers;
pub mod finalize;
pub mod health;
pub mod ingest;

use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/neuropulse".to_string());

    println!("🔌 Conectando ao banco de dados PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = app_router::create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Backend NeuroPulse rodando em http://localhost:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}
