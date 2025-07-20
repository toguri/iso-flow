use nba_trade_scraper::db;
use once_cell::sync::Lazy;
use sqlx::SqlitePool;
use std::sync::Mutex;

// テスト用のデータベースプールを共有
static TEST_DB: Lazy<Mutex<Option<SqlitePool>>> = Lazy::new(|| Mutex::new(None));

pub async fn setup_test_db() -> SqlitePool {
    let mut db = TEST_DB.lock().unwrap();
    if db.is_none() {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        let pool = db::create_pool().await.expect("Failed to create test pool");
        *db = Some(pool.clone());
        pool
    } else {
        db.as_ref().unwrap().clone()
    }
}

pub fn cleanup_test_db() {
    let mut db = TEST_DB.lock().unwrap();
    *db = None;
}
