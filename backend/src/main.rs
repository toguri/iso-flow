mod scraper;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::scraper::RssParser;

#[tokio::main]
async fn main() -> Result<()> {
    // ログの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting NBA Trade Scraper...");

    // RSSフィードの取得テスト
    let parser = RssParser::new();
    let news = parser.fetch_all_feeds().await?;

    info!("Found {} trade-related news items", news.len());

    // 全件を表示（トレード関連度で分類）
    println!("\n📊 トレード関連ニュース分析\n");

    // 明確なトレード情報
    let trades: Vec<_> = news
        .iter()
        .filter(|n| {
            n.title.to_lowercase().contains("trade")
                || n.title.to_lowercase().contains("acquire")
                || n.title.to_lowercase().contains("deal")
        })
        .collect();

    // 契約・サイン関連
    let signings: Vec<_> = news
        .iter()
        .filter(|n| {
            n.title.to_lowercase().contains("sign")
                || n.title.to_lowercase().contains("agree")
                || n.title.to_lowercase().contains("buyout")
        })
        .collect();

    println!("🔄 トレード情報 ({} 件):", trades.len());
    for (i, item) in trades.iter().take(10).enumerate() {
        println!("\n  {}. {}", i + 1, item.title);
        println!("     📅 {}", item.published_at.format("%Y-%m-%d %H:%M"));
        println!("     📰 {}", item.source);
    }

    println!("\n✍️ 契約・サイン情報 ({} 件):", signings.len());
    for (i, item) in signings.iter().take(10).enumerate() {
        println!("\n  {}. {}", i + 1, item.title);
        println!("     📅 {}", item.published_at.format("%Y-%m-%d %H:%M"));
        println!("     📰 {}", item.source);
    }

    println!("\n📈 ソース別統計:");
    println!(
        "  - ESPN: {} 件",
        news.iter()
            .filter(|n| matches!(n.source, crate::scraper::NewsSource::ESPN))
            .count()
    );
    println!(
        "  - RealGM: {} 件",
        news.iter()
            .filter(|n| matches!(n.source, crate::scraper::NewsSource::RealGM))
            .count()
    );

    Ok(())
}
