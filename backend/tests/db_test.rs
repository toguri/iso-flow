use nba_trade_scraper::db;

#[tokio::test]
async fn test_database_setup() {
    // メモリ内DBでテスト
    std::env::set_var("DATABASE_URL", "sqlite::memory:");

    let pool = db::create_pool().await.expect("Failed to create pool");

    // テーブルが作成されているか確認
    let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
        .fetch_all(&pool)
        .await
        .expect("Failed to query tables");

    assert!(result.len() > 0, "No tables were created");

    // trade_newsテーブルが存在するか確認
    let has_trade_news = result.iter().any(|row| {
        let name: String = sqlx::Row::get(row, 0);
        name == "trade_news"
    });

    assert!(has_trade_news, "trade_news table was not created");

    // teams テーブルも確認
    let has_teams = result.iter().any(|row| {
        let name: String = sqlx::Row::get(row, 0);
        name == "teams"
    });

    assert!(has_teams, "teams table was not created");
}
