use chrono::Utc;
use nba_trade_scraper::graphql::TradeNews;
use nba_trade_scraper::scraper::{NewsItem, NewsSource};

#[test]
fn test_news_item_to_trade_news_conversion() {
    let news_item = NewsItem {
        id: "test-id-123".to_string(),
        title: "Lakers Sign New Player".to_string(),
        description: Some("The Lakers have signed a new player to a multi-year deal".to_string()),
        link: "https://example.com/news/123".to_string(),
        source: NewsSource::ESPN,
        category: "Signing".to_string(),
        published_at: Utc::now(),
    };

    let trade_news = TradeNews::from(news_item.clone());

    assert_eq!(trade_news.id, news_item.id);
    assert_eq!(trade_news.title, news_item.title);
    assert_eq!(trade_news.description, news_item.description);
    assert_eq!(trade_news.link, news_item.link);
    assert_eq!(trade_news.source, "ESPN");
    assert_eq!(trade_news.category, news_item.category);
    assert_eq!(trade_news.published_at, news_item.published_at);
}

#[test]
fn test_news_source_variants() {
    // ESPN
    let espn_item = NewsItem {
        id: "espn-1".to_string(),
        title: "ESPN News".to_string(),
        description: None,
        link: "https://espn.com".to_string(),
        source: NewsSource::ESPN,
        category: "Other".to_string(),
        published_at: Utc::now(),
    };
    let espn_trade = TradeNews::from(espn_item);
    assert_eq!(espn_trade.source, "ESPN");

    // RealGM
    let realgm_item = NewsItem {
        id: "realgm-1".to_string(),
        title: "RealGM News".to_string(),
        description: None,
        link: "https://realgm.com".to_string(),
        source: NewsSource::RealGM,
        category: "Trade".to_string(),
        published_at: Utc::now(),
    };
    let realgm_trade = TradeNews::from(realgm_item);
    assert_eq!(realgm_trade.source, "RealGM");

    // HoopsHype
    let hoopshype_item = NewsItem {
        id: "hh-1".to_string(),
        title: "HoopsHype News".to_string(),
        description: None,
        link: "https://hoopshype.com".to_string(),
        source: NewsSource::HoopsHype,
        category: "Signing".to_string(),
        published_at: Utc::now(),
    };
    let hoopshype_trade = TradeNews::from(hoopshype_item);
    assert_eq!(hoopshype_trade.source, "HoopsHype");

    // Other source
    let other_item = NewsItem {
        id: "other-1".to_string(),
        title: "Other Source News".to_string(),
        description: None,
        link: "https://othersource.com".to_string(),
        source: NewsSource::Other("Custom Source".to_string()),
        category: "Other".to_string(),
        published_at: Utc::now(),
    };
    let other_trade = TradeNews::from(other_item);
    assert_eq!(other_trade.source, "Custom Source");
}

#[test]
fn test_optional_description() {
    // With description
    let with_desc = NewsItem {
        id: "1".to_string(),
        title: "Title".to_string(),
        description: Some("Description text".to_string()),
        link: "https://example.com".to_string(),
        source: NewsSource::ESPN,
        category: "Trade".to_string(),
        published_at: Utc::now(),
    };
    let trade_with_desc = TradeNews::from(with_desc);
    assert_eq!(trade_with_desc.description, Some("Description text".to_string()));

    // Without description
    let without_desc = NewsItem {
        id: "2".to_string(),
        title: "Title".to_string(),
        description: None,
        link: "https://example.com".to_string(),
        source: NewsSource::ESPN,
        category: "Trade".to_string(),
        published_at: Utc::now(),
    };
    let trade_without_desc = TradeNews::from(without_desc);
    assert_eq!(trade_without_desc.description, None);
}