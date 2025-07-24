use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use nba_trade_scraper::create_app;
use sqlx::SqlitePool;
use tower::ServiceExt;

#[tokio::test]
async fn test_graphql_playground_endpoint() {
    // テスト用のメモリ内データベース
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // アプリケーションを作成
    let app = create_app(pool);

    // GraphiQLプレイグラウンドにGETリクエスト
    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードを確認
    assert_eq!(response.status(), StatusCode::OK);

    // レスポンスボディにGraphiQLのHTMLが含まれているか確認
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("GraphiQL"));
}

#[tokio::test]
async fn test_graphql_post_endpoint() {
    // テスト用のメモリ内データベース
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // アプリケーションを作成
    let app = create_app(pool);

    // GraphQLクエリを送信
    let query = r#"{"query": "{ __typename }"}"#;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(query))
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードを確認
    assert_eq!(response.status(), StatusCode::OK);

    // レスポンスボディを確認
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Query"));
}

#[tokio::test]
async fn test_cors_headers() {
    // テスト用のメモリ内データベース
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // アプリケーションを作成
    let app = create_app(pool);

    // CORSヘッダーを持つリクエスト
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .header("origin", "http://localhost:3000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // CORSヘッダーが正しく設定されているか確認
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response
        .headers()
        .contains_key("access-control-allow-origin"));
}

#[tokio::test]
async fn test_create_app_routes() {
    // テスト用のメモリ内データベース
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // アプリケーションを作成
    let app = create_app(pool);

    // 存在しないルートへのリクエスト
    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // 404が返ることを確認
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
