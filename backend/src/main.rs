use axum::serve;
use nba_trade_scraper::create_app;
use sqlx::SqlitePool;
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
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:nba_trades.db".to_string());
    let pool = SqlitePool::connect(&database_url).await?;

    // マイグレーション実行
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database initialized");

    let app = create_app(pool);

    info!("GraphQL playground available at http://localhost:8000");

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    serve(listener, app).await?;

    Ok(())
}
