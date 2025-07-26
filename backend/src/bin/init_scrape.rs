//! 初回データ投入用のコマンドラインツール
//!
//! 使用方法: cargo run --bin init_scrape

use anyhow::Result;
use nba_trade_scraper::{
    db::connection::create_pool,
    scraper::{NewsPersistence, RssParser},
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // ログの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting initial RSS scraping...");

    // データベース接続
    let pool = create_pool().await?;

    // RSSフィードを取得
    let parser = RssParser::new();
    let news_items = parser.fetch_all_feeds().await?;
    info!("Fetched {} items from RSS feeds", news_items.len());

    // データベースに保存
    let persistence = NewsPersistence::new(pool);
    let result = persistence.save_news_items(news_items).await?;

    info!(
        "Initial scraping completed: {} saved, {} skipped, {} errors",
        result.saved_count,
        result.skipped_count,
        result.errors.len()
    );

    if !result.errors.is_empty() {
        info!("Errors occurred:");
        for (id, error) in &result.errors {
            info!("  - {}: {}", id, error);
        }
    }

    Ok(())
}
