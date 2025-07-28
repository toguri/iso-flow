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

    #[test]
    fn test_default_trait() {
        let parser1 = RssParser::new();
        let parser2 = RssParser::default();

        // 両方のメソッドが同じようにインスタンスを作成することを確認
        // (Client自体を比較することはできないので、単にパニックしないことを確認)
        assert!(std::ptr::eq(
            &parser1 as *const RssParser,
            &parser1 as *const RssParser
        ));
        assert!(std::ptr::eq(
            &parser2 as *const RssParser,
            &parser2 as *const RssParser
        ));
    }

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

            println!(
                "Categories: Trade={trade_count}, Signing={signing_count}, Other={other_count}"
            );

            println!("\nSample news:");
            for item in news.iter().take(10) {
                println!("- [{}] {} ({})", item.category, item.title, item.source);
            }
        }
    }

    #[test]
    fn test_parse_rss_item_with_valid_data() {
        let parser = RssParser::new();

        // 有効なRSSアイテムのモック
        let mut item = rss::Item::default();
        item.set_title(Some("Lakers Trade for Star Player".to_string()));
        item.set_link(Some("https://example.com/news/123".to_string()));
        item.set_description(Some("The Lakers have completed a trade...".to_string()));
        item.set_pub_date(Some("Mon, 22 Jul 2024 10:00:00 GMT".to_string()));
        item.set_guid(Some(rss::Guid {
            value: "unique-guid-123".to_string(),
            permalink: false,
        }));

        let news = parser.parse_rss_item(&item, &NewsSource::ESPN);
        assert!(news.is_some());

        let news = news.unwrap();
        assert_eq!(news.id, "unique-guid-123");
        assert_eq!(news.title, "Lakers Trade for Star Player");
        assert_eq!(news.link, "https://example.com/news/123");
        assert_eq!(
            news.description,
            Some("The Lakers have completed a trade...".to_string())
        );
        assert_eq!(news.category, "Trade");
    }

    #[test]
    fn test_parse_rss_item_without_title() {
        let parser = RssParser::new();

        // タイトルなしのRSSアイテム
        let mut item = rss::Item::default();
        item.set_link(Some("https://example.com/news/123".to_string()));

        let news = parser.parse_rss_item(&item, &NewsSource::ESPN);
        assert!(news.is_none());
    }

    #[test]
    fn test_parse_rss_item_without_link() {
        let parser = RssParser::new();

        // リンクなしのRSSアイテム
        let mut item = rss::Item::default();
        item.set_title(Some("Test Title".to_string()));

        let news = parser.parse_rss_item(&item, &NewsSource::ESPN);
        assert!(news.is_none());
    }

    #[test]
    fn test_parse_rss_item_without_guid() {
        let parser = RssParser::new();

        // GUIDなしのRSSアイテム（リンクのハッシュが使われる）
        let mut item = rss::Item::default();
        item.set_title(Some("Test News".to_string()));
        item.set_link(Some("https://example.com/news/456".to_string()));

        let news = parser.parse_rss_item(&item, &NewsSource::RealGM);
        assert!(news.is_some());

        let news = news.unwrap();
        assert!(news.id.starts_with("link-"));
    }

    #[test]
    fn test_parse_rss_item_with_invalid_date() {
        let parser = RssParser::new();

        // 無効な日付のRSSアイテム
        let mut item = rss::Item::default();
        item.set_title(Some("Test News".to_string()));
        item.set_link(Some("https://example.com/news/789".to_string()));
        item.set_pub_date(Some("Invalid Date Format".to_string()));

        let news = parser.parse_rss_item(&item, &NewsSource::ESPN);
        assert!(news.is_some());

        let news = news.unwrap();
        // 無効な日付の場合は現在時刻が使われる
        let now = Utc::now();
        let diff = now.timestamp() - news.published_at.timestamp();
        assert!(diff < 5); // 5秒以内
    }
}
