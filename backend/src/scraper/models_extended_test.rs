#[cfg(test)]
mod tests {
    use super::*;
    use crate::scraper::{NewsSource, NewsItem};

    #[test]
    fn test_news_source_display() {
        assert_eq!(NewsSource::ESPN.to_string(), "ESPN");
        assert_eq!(NewsSource::RealGM.to_string(), "RealGM");
        assert_eq!(NewsSource::HoopsHype.to_string(), "HoopsHype");
        assert_eq!(NewsSource::Other("TestSource".to_string()).to_string(), "TestSource");
    }

    #[test]
    fn test_news_source_from_string() {
        assert!(matches!(NewsSource::from_string("ESPN"), NewsSource::ESPN));
        assert!(matches!(NewsSource::from_string("RealGM"), NewsSource::RealGM));
        assert!(matches!(NewsSource::from_string("HoopsHype"), NewsSource::HoopsHype));
        
        match NewsSource::from_string("CustomSource") {
            NewsSource::Other(s) => assert_eq!(s, "CustomSource"),
            _ => panic!("Expected Other variant"),
        }
    }

    #[test]
    fn test_rss_feed_new() {
        let feed = RssFeed::new("https://example.com/rss", NewsSource::ESPN);
        assert_eq!(feed.url, "https://example.com/rss");
        assert!(matches!(feed.source, NewsSource::ESPN));
    }

    #[test]
    fn test_determine_category_edge_cases() {
        // 大文字小文字の混在
        assert_eq!(
            NewsItem::determine_category("LAKERS TRADE Russell Westbrook", None),
            "Trade"
        );

        // 複数キーワード
        assert_eq!(
            NewsItem::determine_category(
                "Team acquires player in blockbuster trade deal",
                None
            ),
            "Trade"
        );

        // 説明文でのキーワード検出
        assert_eq!(
            NewsItem::determine_category(
                "Breaking News",
                Some("The Lakers have agreed to a contract extension with LeBron James")
            ),
            "Signing"
        );

        // Waiveのテスト
        assert_eq!(
            NewsItem::determine_category("Team to waive veteran guard", None),
            "Signing"
        );

        // Re-signのテスト
        assert_eq!(
            NewsItem::determine_category("Star player to re-sign with team", None),
            "Signing"
        );

        // Option関連のテスト
        assert_eq!(
            NewsItem::determine_category("Team will pick up option on rookie", None),
            "Signing"
        );
        assert_eq!(
            NewsItem::determine_category("Player to decline option", None),
            "Signing"
        );
    }

    #[test]
    fn test_generate_id_consistency() {
        // 同じリンクから生成されるIDは常に同じ
        let link = "https://example.com/news/article-123";
        let id1 = NewsItem::generate_id(None, link);
        let id2 = NewsItem::generate_id(None, link);
        assert_eq!(id1, id2);

        // 異なるリンクからは異なるID
        let link2 = "https://example.com/news/article-456";
        let id3 = NewsItem::generate_id(None, link2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_rss_feeds_constant() {
        // RSS_FEEDS定数が正しく定義されていることを確認
        assert_eq!(RSS_FEEDS.len(), 2);
        
        // ESPN feed
        assert_eq!(RSS_FEEDS[0].0, "https://www.espn.com/espn/rss/nba/news");
        assert!(matches!(RSS_FEEDS[0].1, NewsSource::ESPN));

        // RealGM feed
        assert_eq!(RSS_FEEDS[1].0, "https://basketball.realgm.com/rss/wiretap/0/0.xml");
        assert!(matches!(RSS_FEEDS[1].1, NewsSource::RealGM));
    }

    #[test]
    fn test_news_item_serialization() {
        use chrono::Utc;
        
        let news_item = NewsItem {
            id: "test-123".to_string(),
            title: "Test Title".to_string(),
            description: Some("Test Description".to_string()),
            link: "https://example.com".to_string(),
            source: NewsSource::ESPN,
            category: "Trade".to_string(),
            published_at: Utc::now(),
        };

        // Serialize
        let serialized = serde_json::to_string(&news_item).unwrap();
        assert!(serialized.contains("test-123"));
        assert!(serialized.contains("Test Title"));

        // Deserialize
        let deserialized: NewsItem = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.id, news_item.id);
        assert_eq!(deserialized.title, news_item.title);
    }
}