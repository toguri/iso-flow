use axum::serve;
use nba_trade_scraper::{create_app, db::connection::create_pool};
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ログの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting GraphQL server...");

    // データベース接続の初期化
    let pool = create_pool().await?;
    
    // DATABASE_URLから適切なマイグレーションを実行
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:nba_trades.db".to_string());
    
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        info!("Running PostgreSQL migrations...");
        sqlx::migrate!("./migrations_postgres").run(&pool).await?;
    } else {
        info!("Running SQLite migrations...");
        sqlx::migrate!("./migrations").run(&pool).await?;
    }

    info!("Database initialized");

    let app = create_app(pool);

    info!("GraphQL playground available at http://localhost:8000");

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    serve(listener, app).await?;

    Ok(())
}
