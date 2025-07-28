use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use nba_trade_scraper::create_app;
use tower::ServiceExt;

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_health_check_endpoint() {
    // PostgreSQLデータベースが必要
    let pool = match nba_trade_scraper::db::connection::create_pool().await {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping test: PostgreSQL database connection required");
            return;
        }
    };
    // 結合テスト環境ではinit.sqlでテーブルが作成されるため、マイグレーションはスキップ
    // sqlx::migrate!("./migrations_postgres")
    //     .run(&pool)
    //     .await
    //     .unwrap();

    // アプリケーションを作成
    let app = create_app(pool);

    // ヘルスチェックエンドポイントにリクエスト
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードを確認
    assert_eq!(response.status(), StatusCode::OK);

    // レスポンスボディを確認
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // JSONレスポンスを確認
    let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["service"], "nba-trade-scraper");
}
