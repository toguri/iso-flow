mod common;

use chrono::Utc;
use nba_trade_scraper::db::models::{NewTradeNews, TradeNewsDb};

#[tokio::test]
async fn test_insert_and_retrieve_trade_news() {
    let pool = common::setup_test_db().await;

    // テストデータを作成
    let new_news = NewTradeNews {
        external_id: "test-001".to_string(),
        title: "Test Trade News".to_string(),
        description: Some("This is a test trade news item".to_string()),
        source_name: "Test Source".to_string(),
        source_url: "https://example.com/test".to_string(),
        author: Some("Test Author".to_string()),
        category: "Trade".to_string(),
        is_official: false,
        published_at: Utc::now(),
    };

    // データを挿入
    let insert_result = sqlx::query(
        r#"
        INSERT INTO trade_news (
            external_id, title, description, source_name, source_url,
            author, category, is_official, published_at, scraped_at,
            translation_status, created_at, updated_at
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#,
    )
    .bind(&new_news.external_id)
    .bind(&new_news.title)
    .bind(&new_news.description)
    .bind(&new_news.source_name)
    .bind(&new_news.source_url)
    .bind(&new_news.author)
    .bind(&new_news.category)
    .bind(&new_news.is_official)
    .bind(&new_news.published_at)
    .bind(Utc::now())
    .bind("pending")
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(&pool)
    .await
    .expect("Failed to insert trade news");

    assert_eq!(insert_result.rows_affected(), 1);

    // データを取得
    let retrieved: TradeNewsDb = sqlx::query_as(
        "SELECT * FROM trade_news WHERE external_id = ?",
    )
    .bind(&new_news.external_id)
    .fetch_one(&pool)
    .await
    .expect("Failed to retrieve trade news");

    // 検証
    assert_eq!(retrieved.external_id, new_news.external_id);
    assert_eq!(retrieved.title, new_news.title);
    assert_eq!(retrieved.description, new_news.description);
    assert_eq!(retrieved.source_name, new_news.source_name);
    assert_eq!(retrieved.category, new_news.category);
}

#[tokio::test]
async fn test_unique_constraint() {
    let pool = common::setup_test_db().await;

    let new_news = NewTradeNews {
        external_id: "duplicate-001".to_string(),
        title: "Duplicate Test".to_string(),
        description: None,
        source_name: "Test".to_string(),
        source_url: "https://example.com/dup".to_string(),
        author: None,
        category: "Trade".to_string(),
        is_official: false,
        published_at: Utc::now(),
    };

    // 最初の挿入は成功
    let first_insert = sqlx::query(
        r#"
        INSERT INTO trade_news (
            external_id, title, source_name, source_url,
            category, is_official, published_at, scraped_at,
            translation_status, created_at, updated_at
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#,
    )
    .bind(&new_news.external_id)
    .bind(&new_news.title)
    .bind(&new_news.source_name)
    .bind(&new_news.source_url)
    .bind(&new_news.category)
    .bind(&new_news.is_official)
    .bind(&new_news.published_at)
    .bind(Utc::now())
    .bind("pending")
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(&pool)
    .await;

    assert!(first_insert.is_ok());

    // 同じexternal_idで再度挿入しようとするとエラー
    let duplicate_insert = sqlx::query(
        r#"
        INSERT INTO trade_news (
            external_id, title, source_name, source_url,
            category, is_official, published_at, scraped_at,
            translation_status, created_at, updated_at
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#,
    )
    .bind(&new_news.external_id)
    .bind("Different Title")
    .bind(&new_news.source_name)
    .bind(&new_news.source_url)
    .bind(&new_news.category)
    .bind(&new_news.is_official)
    .bind(&new_news.published_at)
    .bind(Utc::now())
    .bind("pending")
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(&pool)
    .await;

    assert!(duplicate_insert.is_err());
}

#[tokio::test]
async fn test_query_by_category() {
    let pool = common::setup_test_db().await;

    // 異なるカテゴリーのテストデータを挿入
    let categories = vec!["Trade", "Signing", "Other"];
    for (i, category) in categories.iter().enumerate() {
        let _ = sqlx::query(
            r#"
            INSERT INTO trade_news (
                external_id, title, source_name, source_url,
                category, is_official, published_at, scraped_at,
                translation_status, created_at, updated_at
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
        )
        .bind(format!("cat-test-{}", i))
        .bind(format!("{} News", category))
        .bind("Test")
        .bind("https://example.com")
        .bind(category)
        .bind(false)
        .bind(Utc::now())
        .bind(Utc::now())
        .bind("pending")
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&pool)
        .await
        .expect("Failed to insert test data");
    }

    // Tradeカテゴリーのニュースを取得
    let trade_news: Vec<TradeNewsDb> = sqlx::query_as(
        "SELECT * FROM trade_news WHERE category = ?",
    )
    .bind("Trade")
    .fetch_all(&pool)
    .await
    .expect("Failed to query by category");

    assert!(!trade_news.is_empty());
    assert!(trade_news.iter().all(|n| n.category == "Trade"));
}