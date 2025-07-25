use anyhow::Result;
use clap::Parser;
use nba_trade_scraper::{
    db::connection::create_pool,
    scheduler::{create_scheduler, run_scraping_job},
};
use tokio::signal;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "scheduler")]
#[command(about = "NBAニュースの定期スクレイピングスケジューラー")]
struct Cli {
    /// 起動時に即座にスクレイピングを実行
    #[arg(short, long)]
    immediate: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // ログの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    info!("Starting NBA Trade Scraper Scheduler...");

    // データベース接続（PostgreSQL）
    let pool = create_pool().await?;

    info!("Database connected");

    // 起動時に即座に実行するオプション
    if cli.immediate {
        info!("Running immediate scraping job...");
        match run_scraping_job(pool.clone()).await {
            Ok(_) => info!("Initial scraping completed successfully"),
            Err(e) => error!("Initial scraping failed: {}", e),
        }
    }

    // スケジューラー作成・開始
    let mut scheduler = create_scheduler(pool).await?;
    scheduler.start().await?;

    info!("Scheduler started. Press Ctrl+C to stop.");
    info!("Jobs will run every 5 minutes at :00, :05, :10, :15, etc.");

    // Ctrl+Cを待つ
    signal::ctrl_c().await?;

    info!("Shutting down scheduler...");
    scheduler.shutdown().await?;
    info!("Scheduler stopped successfully");

    Ok(())
}
