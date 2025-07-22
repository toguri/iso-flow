use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;
use rss::Channel;
use tracing::{debug, error, info};

use crate::scraper::models::{NewsItem, NewsSource, RssFeed, RSS_FEEDS};

pub struct RssParser {
    client: Client,
}

impl Default for RssParser {
    fn default() -> Self {
        Self::new()
    }
}

impl RssParser {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn fetch_all_feeds(&self) -> Result<Vec<NewsItem>> {
        let mut all_news = Vec::new();

        for (url, source) in RSS_FEEDS {
            match self.fetch_feed(&RssFeed::new(url, source.clone())).await {
                Ok(mut news) => {
                    info!("Fetched {} items from {}", news.len(), source);
                    all_news.append(&mut news);
                }
                Err(e) => {
                    error!("Failed to fetch feed from {}: {}", source, e);
                }
            }
        }

        all_news.sort_by(|a, b| b.published_at.cmp(&a.published_at));
        Ok(all_news)
    }

    pub async fn fetch_feed(&self, feed: &RssFeed) -> Result<Vec<NewsItem>> {
        info!("Fetching RSS feed from: {}", feed.url);

        let response = self
            .client
            .get(&feed.url)
            .send()
            .await
            .context("Failed to fetch RSS feed")?;

        let content = response
            .text()
            .await
            .context("Failed to read response body")?;

        let channel = Channel::read_from(content.as_bytes()).context("Failed to parse RSS feed")?;

        let mut news_items = Vec::new();

        for item in channel.items() {
            if let Some(news) = self.parse_rss_item(item, &feed.source) {
                debug!("Found news item: {} [{}]", news.title, news.category);
                news_items.push(news);
            }
        }

        Ok(news_items)
    }

    fn parse_rss_item(&self, item: &rss::Item, source: &NewsSource) -> Option<NewsItem> {
        let title = item.title()?.to_string();
        let link = item.link()?.to_string();
        let description = item.description().map(|s| s.to_string());

        let published_at = item
            .pub_date()
            .and_then(|date| DateTime::parse_from_rfc2822(date).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        // GUIDからIDを生成、なければリンクのハッシュを使用
        let guid = item.guid().map(|g| g.value());
        let id = NewsItem::generate_id(guid, &link);

        // カテゴリーを判定
        let category = NewsItem::determine_category(&title, description.as_deref());

        Some(NewsItem {
            id,
            title,
            description,
            link,
            published_at,
            source: source.clone(),
            category,
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_feeds() {
        let parser = RssParser::new();
        let result = parser.fetch_all_feeds().await;

        assert!(result.is_ok());
        let news = result.unwrap();

        if !news.is_empty() {
            println!("Found {} news items", news.len());
            
            // カテゴリー別に集計
            let trade_count = news.iter().filter(|n| n.category == "Trade").count();
            let signing_count = news.iter().filter(|n| n.category == "Signing").count();
            let other_count = news.iter().filter(|n| n.category == "Other").count();
            
            println!("Categories: Trade={}, Signing={}, Other={}", trade_count, signing_count, other_count);
            
            println!("\nSample news:");
            for item in news.iter().take(10) {
                println!("- [{}] {} ({})", item.category, item.title, item.source);
            }
        }
    }
}
