use nba_trade_scraper::scraper::{models::NewsSource, rss_parser::RssParser};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_parse_rss_feed() {
    // モックサーバーを起動
    let mock_server = MockServer::start().await;

    // モックRSSレスポンスを作成
    let mock_rss = r#"<?xml version="1.0" encoding="UTF-8"?>
    <rss version="2.0">
        <channel>
            <title>NBA News</title>
            <link>http://example.com</link>
            <description>Latest NBA News</description>
            <item>
                <title>Lakers trade for star player</title>
                <link>http://example.com/lakers-trade</link>
                <description>The Los Angeles Lakers have acquired a new star player in a blockbuster trade.</description>
                <pubDate>Wed, 01 Jan 2025 12:00:00 GMT</pubDate>
            </item>
            <item>
                <title>Warriors sign veteran guard</title>
                <link>http://example.com/warriors-sign</link>
                <description>Golden State Warriors sign experienced guard to strengthen their roster.</description>
                <pubDate>Wed, 01 Jan 2025 11:00:00 GMT</pubDate>
            </item>
            <item>
                <title>NBA playoff predictions</title>
                <link>http://example.com/playoff-predictions</link>
                <description>Expert analysis of this year's playoff contenders.</description>
                <pubDate>Wed, 01 Jan 2025 10:00:00 GMT</pubDate>
            </item>
        </channel>
    </rss>"#;

    // モックエンドポイントを設定
    Mock::given(method("GET"))
        .and(path("/nba.rss"))
        .respond_with(ResponseTemplate::new(200).set_body_string(mock_rss))
        .mount(&mock_server)
        .await;

    // RssParserを作成してフィードを取得
    let parser = RssParser::new();
    let feed_url = format!("{}/nba.rss", &mock_server.uri());
    let news_items = parser
        .fetch_feed(&nba_trade_scraper::scraper::models::RssFeed::new(
            &feed_url,
            NewsSource::ESPN,
        ))
        .await
        .expect("Failed to fetch feed");

    // 結果を検証 - 全てのニュースが取得される（フィルタリングは行わない）
    assert_eq!(news_items.len(), 3);

    // トレード記事が正しく取得されているか確認
    let trade_news = news_items
        .iter()
        .find(|n| n.title.contains("Lakers trade"))
        .expect("Trade news not found");
    assert_eq!(trade_news.title, "Lakers trade for star player");
    assert_eq!(trade_news.category, "Trade");

    // サイン記事が正しく取得されているか確認
    let signing_news = news_items
        .iter()
        .find(|n| n.title.contains("Warriors sign"))
        .expect("Signing news not found");
    assert_eq!(signing_news.title, "Warriors sign veteran guard");
    assert_eq!(signing_news.category, "Signing");

    // Other記事も取得される
    let other_news = news_items
        .iter()
        .find(|n| n.title.contains("playoff predictions"))
        .expect("Other news not found");
    assert_eq!(other_news.category, "Other");
}

#[tokio::test]
async fn test_handle_invalid_rss() {
    let mock_server = MockServer::start().await;

    // 無効なRSSを返す
    Mock::given(method("GET"))
        .and(path("/invalid.rss"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Not valid RSS"))
        .mount(&mock_server)
        .await;

    let parser = RssParser::new();
    let feed_url = format!("{}/invalid.rss", &mock_server.uri());
    let result = parser
        .fetch_feed(&nba_trade_scraper::scraper::models::RssFeed::new(
            &feed_url,
            NewsSource::ESPN,
        ))
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_handle_http_error() {
    let mock_server = MockServer::start().await;

    // 404エラーを返す
    Mock::given(method("GET"))
        .and(path("/notfound.rss"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let parser = RssParser::new();
    let feed_url = format!("{}/notfound.rss", &mock_server.uri());
    let result = parser
        .fetch_feed(&nba_trade_scraper::scraper::models::RssFeed::new(
            &feed_url,
            NewsSource::ESPN,
        ))
        .await;

    assert!(result.is_err());
}
