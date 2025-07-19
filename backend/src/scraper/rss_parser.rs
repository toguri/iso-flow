use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;
use rss::Channel;
use tracing::{debug, error, info};

use crate::scraper::models::{NewsSource, RssFeed, TradeNews, RSS_FEEDS};

pub struct RssParser {
    client: Client,
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

    pub async fn fetch_all_feeds(&self) -> Result<Vec<TradeNews>> {
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

    pub async fn fetch_feed(&self, feed: &RssFeed) -> Result<Vec<TradeNews>> {
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
                if self.is_trade_related(&news) {
                    debug!("Found trade-related news: {}", news.title);
                    news_items.push(news);
                }
            }
        }

        Ok(news_items)
    }

    fn parse_rss_item(&self, item: &rss::Item, source: &NewsSource) -> Option<TradeNews> {
        let title = item.title()?.to_string();
        let link = item.link()?.to_string();
        let description = item.description().unwrap_or("").to_string();

        let published_at = item
            .pub_date()
            .and_then(|date| DateTime::parse_from_rfc2822(date).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let id = item
            .guid()
            .map(|g| g.value().to_string())
            .unwrap_or_else(|| format!("{}-{}", source, published_at.timestamp()));

        Some(TradeNews {
            id,
            title,
            description,
            link,
            published_at,
            source: source.clone(),
            is_trade: false, // Will be determined by is_trade_related
        })
    }

    fn is_trade_related(&self, news: &TradeNews) -> bool {
        let keywords = vec![
            "trade", "traded", "acquire", "acquired", "deal", "sign", "waive", "waived", "buyout",
            "release", "released", "exchange", "send", "sent", "receive", "swap",
        ];

        let text = format!(
            "{} {}",
            news.title.to_lowercase(),
            news.description.to_lowercase()
        );

        keywords.iter().any(|keyword| text.contains(keyword))
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
            println!("Found {} trade-related news items", news.len());
            for item in news.iter().take(5) {
                println!("- {} ({})", item.title, item.source);
            }
        }
    }
}
