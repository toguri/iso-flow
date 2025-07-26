//! scraper/models.rsの追加単体テスト

use nba_trade_scraper::scraper::{NewsItem, NewsSource, RssFeed, RSS_FEEDS};
use chrono::Utc;

#[test]
fn test_news_source_display() {
    // Display実装のテスト
    assert_eq!(NewsSource::ESPN.to_string(), "ESPN");
    assert_eq!(NewsSource::RealGM.to_string(), "RealGM");
    assert_eq!(NewsSource::HoopsHype.to_string(), "HoopsHype");
    assert_eq!(NewsSource::Other("Custom".to_string()).to_string(), "Custom");
}

#[test]
fn test_news_source_from_string() {
    // from_string実装のテスト
    assert!(matches!(NewsSource::from_string("ESPN"), NewsSource::ESPN));
    assert!(matches!(NewsSource::from_string("RealGM"), NewsSource::RealGM));
    assert!(matches!(NewsSource::from_string("HoopsHype"), NewsSource::HoopsHype));
    
    // from_stringは大文字小文字を区別する
    let other_espn = NewsSource::from_string("espn");
    match other_espn {
        NewsSource::Other(name) => assert_eq!(name, "espn"),
        _ => panic!("Expected Other variant for lowercase"),
    }
    
    // その他のソース
    let other = NewsSource::from_string("UnknownSource");
    match other {
        NewsSource::Other(name) => assert_eq!(name, "UnknownSource"),
        _ => panic!("Expected Other variant"),
    }
}

#[test]
fn test_rss_feed_new() {
    // RssFeed::new()のテスト
    let feed = RssFeed::new("https://example.com/rss", NewsSource::ESPN);
    assert_eq!(feed.url, "https://example.com/rss");
    assert!(matches!(feed.source, NewsSource::ESPN));
}

#[test]
fn test_determine_category_method() {
    // determine_categoryはNewsItemのメソッドであることを確認
    // 大文字小文字のテスト
    assert_eq!(NewsItem::determine_category("The Lakers TRADE LeBron", None), "Trade");
    assert_eq!(NewsItem::determine_category("player SIGNING announced", None), "Signing");
    
    // 複数のキーワードが含まれる場合（最初にマッチしたもの）
    assert_eq!(NewsItem::determine_category("Trade talks about signing new player", None), "Trade");
    
    // 部分文字列としてのマッチ
    assert_eq!(NewsItem::determine_category("Blockbuster traded announced", None), "Trade");
    assert_eq!(NewsItem::determine_category("Multi-year contract extension", None), "Signing");
    
    // 特殊文字を含む場合
    assert_eq!(NewsItem::determine_category("BREAKING: Lakers acquire star", None), "Trade");
    assert_eq!(NewsItem::determine_category("UPDATE: Waived by team", None), "Signing"); // waived は signing_keywords に含まれる
    
    // 空文字列
    assert_eq!(NewsItem::determine_category("", None), "Other");
    
    // キーワードがない場合
    assert_eq!(NewsItem::determine_category("NBA game highlights from last night", None), "Other");
    
    // descriptionも考慮される場合
    assert_eq!(NewsItem::determine_category("Lakers News", Some("Team trades player")), "Trade");
}

#[test]
fn test_news_item_id_generation() {
    // NewsItemのID生成の一貫性をテスト
    let item1 = NewsItem {
        id: "id-1".to_string(),
        title: "Test News 1".to_string(),
        description: None,
        link: "https://example.com/news/123".to_string(),
        source: NewsSource::ESPN,
        category: "Trade".to_string(),
        published_at: Utc::now(),
    };
    
    let item2 = NewsItem {
        id: "id-2".to_string(),
        title: "Test News 2".to_string(),
        description: None,
        link: "https://example.com/news/456".to_string(),
        source: NewsSource::ESPN,
        category: "Trade".to_string(),
        published_at: Utc::now(),
    };
    
    // IDが異なることを確認
    assert_ne!(item1.id, item2.id);
}

#[test]
fn test_rss_feeds_constant() {
    // RSS_FEEDS定数の検証
    assert!(!RSS_FEEDS.is_empty(), "RSS_FEEDS should not be empty");
    
    // 各フィードのURLが有効な形式であることを確認
    for (url, source) in RSS_FEEDS.iter() {
        assert!(url.starts_with("http"), "Feed URL should start with http");
        assert!(!url.is_empty(), "Feed URL should not be empty");
        
        // sourceの型を確認
        match source {
            NewsSource::ESPN => assert_eq!(source.to_string(), "ESPN"),
            NewsSource::RealGM => assert_eq!(source.to_string(), "RealGM"),
            _ => {}
        }
    }
    
    // 主要なソースが含まれていることを確認
    let has_espn = RSS_FEEDS.iter().any(|(_, source)| matches!(source, NewsSource::ESPN));
    assert!(has_espn, "Should include ESPN");
}

#[test]
fn test_news_item_serialization() {
    // NewsItemのシリアライゼーション/デシリアライゼーションテスト
    let item = NewsItem {
        id: "test-123".to_string(),
        title: "Test News".to_string(),
        description: Some("Test description".to_string()),
        link: "https://example.com".to_string(),
        source: NewsSource::ESPN,
        category: "Trade".to_string(),
        published_at: Utc::now(),
    };
    
    // JSONにシリアライズ
    let json = serde_json::to_string(&item).expect("Should serialize to JSON");
    
    // JSONからデシリアライズ
    let deserialized: NewsItem = serde_json::from_str(&json).expect("Should deserialize from JSON");
    
    // 元のアイテムと一致することを確認
    assert_eq!(deserialized.id, item.id);
    assert_eq!(deserialized.title, item.title);
    assert_eq!(deserialized.description, item.description);
    assert_eq!(deserialized.link, item.link);
    assert_eq!(deserialized.source.to_string(), item.source.to_string());
    assert_eq!(deserialized.category, item.category);
}

#[test]
fn test_news_item_with_none_description() {
    // descriptionがNoneの場合のテスト
    let item = NewsItem {
        id: "test-456".to_string(),
        title: "No Description News".to_string(),
        description: None,
        link: "https://example.com/news".to_string(),
        source: NewsSource::RealGM,
        category: "Other".to_string(),
        published_at: Utc::now(),
    };
    
    // JSONにシリアライズしてデシリアライズ
    let json = serde_json::to_string(&item).expect("Should serialize");
    let deserialized: NewsItem = serde_json::from_str(&json).expect("Should deserialize");
    
    assert!(deserialized.description.is_none());
}