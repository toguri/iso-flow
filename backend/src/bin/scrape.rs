use nba_trade_scraper::{
    db::connection::create_pool,
    scraper::{NewsPersistence, RssParser},
};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ログの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting news scraping...");

    // データベース接続
    let pool = create_pool().await?;

    // RSSフィードからニュースを取得
    let parser = RssParser::new();
    let news_items = parser.fetch_all_feeds().await?;

    info!("Fetched {} news items", news_items.len());

    // データベースに保存
    let persistence = NewsPersistence::new(pool);
    let result = persistence.save_news_items(news_items).await?;

    info!(
        "Scraping completed: {} saved, {} skipped, {} errors",
        result.saved_count,
        result.skipped_count,
        result.errors.len()
    );

    if !result.errors.is_empty() {
        error!("Errors encountered:");
        for (id, error) in result.errors {
            error!("  {}: {}", id, error);
        }
    }

    Ok(())
}
