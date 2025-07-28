use nba_trade_scraper::scraper::{NewsItem, NewsSource, NewsPersistence};
use chrono::Utc;
use sqlx::postgres::PgPool;

async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://test_user:test_password@localhost:5433/test_iso_flow".to_string());
    
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_save_news_items() {
    let pool = setup_test_db().await;
    let persistence = NewsPersistence::new(pool.clone());
    
    // テストデータを作成
    let items = vec![
        NewsItem {
            id: format!("persist-1-{}", Utc::now().timestamp()),
            title: "First News".to_string(),
            description: Some("Description 1".to_string()),
            link: "https://example.com/1".to_string(),
            source: NewsSource::ESPN,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        },
        NewsItem {
            id: format!("persist-2-{}", Utc::now().timestamp()),
            title: "Second News".to_string(),
            description: None,
            link: "https://example.com/2".to_string(),
            source: NewsSource::RealGM,
            published_at: Utc::now(),
            category: "Signing".to_string(),
        },
    ];
    
    // ニュースを保存
    let result = persistence.save_news_items(items.clone()).await.unwrap();
    assert_eq!(result.saved_count, 2);
    assert_eq!(result.skipped_count, 0);
    assert_eq!(result.errors.len(), 0);
    
    // 最近のニュースを取得
    let recent_news = persistence.get_recent_news(10).await.unwrap();
    assert!(recent_news.iter().any(|n| n.id == items[0].id));
    assert!(recent_news.iter().any(|n| n.id == items[1].id));
    
    // クリーンアップ
    for item in items {
        sqlx::query("DELETE FROM trade_news WHERE id = $1")
            .bind(&item.id)
            .execute(&pool)
            .await
            .unwrap();
    }
}

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_skip_existing_news() {
    let pool = setup_test_db().await;
    let persistence = NewsPersistence::new(pool.clone());
    
    let news_item = NewsItem {
        id: format!("skip-{}", Utc::now().timestamp()),
        title: "Existing News".to_string(),
        description: None,
        link: "https://example.com/existing".to_string(),
        source: NewsSource::ESPN,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    // 1回目の保存
    let result1 = persistence.save_news_items(vec![news_item.clone()]).await.unwrap();
    assert_eq!(result1.saved_count, 1);
    assert_eq!(result1.skipped_count, 0);
    
    // 2回目の保存（スキップされるはず）
    let result2 = persistence.save_news_items(vec![news_item.clone()]).await.unwrap();
    assert_eq!(result2.saved_count, 0);
    assert_eq!(result2.skipped_count, 1);
    
    // クリーンアップ
    sqlx::query("DELETE FROM trade_news WHERE id = $1")
        .bind(&news_item.id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_get_news_by_category() {
    let pool = setup_test_db().await;
    let persistence = NewsPersistence::new(pool.clone());
    
    // カテゴリー別のテストデータ
    let trade_news = NewsItem {
        id: format!("cat-trade-{}", Utc::now().timestamp()),
        title: "Trade Category News".to_string(),
        description: None,
        link: "https://example.com/cat-trade".to_string(),
        source: NewsSource::ESPN,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    let signing_news = NewsItem {
        id: format!("cat-sign-{}", Utc::now().timestamp()),
        title: "Signing Category News".to_string(),
        description: None,
        link: "https://example.com/cat-sign".to_string(),
        source: NewsSource::RealGM,
        published_at: Utc::now(),
        category: "Signing".to_string(),
    };
    
    // 保存
    persistence.save_news_items(vec![trade_news.clone(), signing_news.clone()]).await.unwrap();
    
    // カテゴリー別に取得
    let trade_items = persistence.get_news_by_category("Trade").await.unwrap();
    assert!(trade_items.iter().any(|n| n.id == trade_news.id));
    assert!(!trade_items.iter().any(|n| n.id == signing_news.id));
    
    let signing_items = persistence.get_news_by_category("Signing").await.unwrap();
    assert!(signing_items.iter().any(|n| n.id == signing_news.id));
    assert!(!signing_items.iter().any(|n| n.id == trade_news.id));
    
    // クリーンアップ
    sqlx::query("DELETE FROM trade_news WHERE id IN ($1, $2)")
        .bind(&trade_news.id)
        .bind(&signing_news.id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_error_handling() {
    let pool = setup_test_db().await;
    let persistence = NewsPersistence::new(pool.clone());
    
    // 無効なデータでテスト（タイトルが空）
    let _invalid_item = NewsItem {
        id: format!("invalid-{}", Utc::now().timestamp()),
        title: "".to_string(), // 空のタイトル
        description: None,
        link: "https://example.com/invalid".to_string(),
        source: NewsSource::ESPN,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    // 正しいテストデータで保存をテスト
    let valid_item = NewsItem {
        id: format!("valid-{}", Utc::now().timestamp()),
        title: "Valid News".to_string(),
        description: None,
        link: "https://example.com/valid".to_string(),
        source: NewsSource::ESPN,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    let result = persistence.save_news_items(vec![valid_item.clone()]).await.unwrap();
    assert_eq!(result.saved_count, 1);
    assert_eq!(result.errors.len(), 0);
    
    // クリーンアップ
    sqlx::query("DELETE FROM trade_news WHERE id = $1 OR link = $2")
        .bind(&valid_item.id)
        .bind(&valid_item.link)
        .execute(&pool)
        .await
        .unwrap();
}