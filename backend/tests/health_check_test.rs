use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use nba_trade_scraper::create_app;
use sqlx::SqlitePool;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_check_endpoint() {
    // テスト用のメモリ内データベース
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

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
    assert_eq!(body_str, "OK");
}
