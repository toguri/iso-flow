use nba_trade_scraper::db::repository::PgNewsRepository;
use nba_trade_scraper::scraper::{NewsItem, NewsSource};
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
async fn test_save_and_retrieve_news() {
    let pool = setup_test_db().await;
    let repo = PgNewsRepository::new(pool.clone());
    
    // テストデータを作成
    let test_item = NewsItem {
        id: format!("test-{}", Utc::now().timestamp()),
        title: "LeBron James traded to Lakers".to_string(),
        description: Some("Big trade news".to_string()),
        link: "https://example.com/news/1".to_string(),
        source: NewsSource::ESPN,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    // ニュースを保存
    repo.save_news(vec![test_item.clone()]).await.unwrap();
    
    // 全てのニュースを取得
    let all_news = repo.get_all_news().await.unwrap();
    assert!(all_news.iter().any(|item| item.id == test_item.id));
    
    // クリーンアップ
    sqlx::query("DELETE FROM trade_news WHERE id = $1")
        .bind(&test_item.id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_get_news_by_category() {
    let pool = setup_test_db().await;
    let repo = PgNewsRepository::new(pool.clone());
    
    // テストデータを作成
    let trade_item = NewsItem {
        id: format!("trade-{}", Utc::now().timestamp()),
        title: "Trade News".to_string(),
        description: None,
        link: "https://example.com/trade".to_string(),
        source: NewsSource::ESPN,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    let signing_item = NewsItem {
        id: format!("signing-{}", Utc::now().timestamp()),
        title: "Signing News".to_string(),
        description: None,
        link: "https://example.com/signing".to_string(),
        source: NewsSource::RealGM,
        published_at: Utc::now(),
        category: "Signing".to_string(),
    };
    
    // ニュースを保存
    repo.save_news(vec![trade_item.clone(), signing_item.clone()]).await.unwrap();
    
    // カテゴリー別に取得
    let trade_news = repo.get_news_by_category("Trade").await.unwrap();
    assert!(trade_news.iter().any(|item| item.id == trade_item.id));
    
    let signing_news = repo.get_news_by_category("Signing").await.unwrap();
    assert!(signing_news.iter().any(|item| item.id == signing_item.id));
    
    // クリーンアップ
    sqlx::query("DELETE FROM trade_news WHERE id IN ($1, $2)")
        .bind(&trade_item.id)
        .bind(&signing_item.id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_get_news_by_source() {
    let pool = setup_test_db().await;
    let repo = PgNewsRepository::new(pool.clone());
    
    // テストデータを作成
    let espn_item = NewsItem {
        id: format!("espn-{}", Utc::now().timestamp()),
        title: "ESPN News".to_string(),
        description: None,
        link: "https://espn.com/news".to_string(),
        source: NewsSource::ESPN,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    let realgm_item = NewsItem {
        id: format!("realgm-{}", Utc::now().timestamp()),
        title: "RealGM News".to_string(),
        description: None,
        link: "https://realgm.com/news".to_string(),
        source: NewsSource::RealGM,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    // ニュースを保存
    repo.save_news(vec![espn_item.clone(), realgm_item.clone()]).await.unwrap();
    
    // ソース別に取得
    let espn_news = repo.get_news_by_source("ESPN").await.unwrap();
    assert!(espn_news.iter().any(|item| item.id == espn_item.id));
    
    let realgm_news = repo.get_news_by_source("RealGM").await.unwrap();
    assert!(realgm_news.iter().any(|item| item.id == realgm_item.id));
    
    // クリーンアップ
    sqlx::query("DELETE FROM trade_news WHERE id IN ($1, $2)")
        .bind(&espn_item.id)
        .bind(&realgm_item.id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_duplicate_news_handling() {
    let pool = setup_test_db().await;
    let repo = PgNewsRepository::new(pool.clone());
    
    // 同じIDのニュースを2つ作成
    let news_item = NewsItem {
        id: format!("dup-{}", Utc::now().timestamp()),
        title: "Duplicate Test".to_string(),
        description: None,
        link: "https://example.com/dup".to_string(),
        source: NewsSource::ESPN,
        published_at: Utc::now(),
        category: "Trade".to_string(),
    };
    
    // 1回目の保存
    repo.save_news(vec![news_item.clone()]).await.unwrap();
    
    // 2回目の保存（重複）
    let result = repo.save_news(vec![news_item.clone()]).await;
    assert!(result.is_ok()); // ON CONFLICT DO NOTHINGなので成功する
    
    // ニュースが1つだけ存在することを確認
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM trade_news WHERE id = $1")
        .bind(&news_item.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    
    // クリーンアップ
    sqlx::query("DELETE FROM trade_news WHERE id = $1")
        .bind(&news_item.id)
        .execute(&pool)
        .await
        .unwrap();
}