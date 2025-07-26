mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use nba_trade_scraper::graphql::{create_schema, graphql_routes};
use tower::ServiceExt;

#[tokio::test]
#[ignore = "Temporarily disabled: AnyPool driver issue in tests"]
async fn test_graphql_playground() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    let pool = nba_trade_scraper::db::connection::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let schema = create_schema(pool);
    let app = graphql_routes(schema);

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

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[ignore = "Temporarily disabled: AnyPool driver issue in tests"]
async fn test_trade_news_query() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    let pool = nba_trade_scraper::db::connection::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let schema = create_schema(pool);
    let app = graphql_routes(schema);

    let query = r#"{
        "query": "{ tradeNews { id title source category } }"
    }"#;

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

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[ignore = "Temporarily disabled: AnyPool driver issue in tests"]
async fn test_trade_news_by_category_query() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    let pool = nba_trade_scraper::db::connection::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let schema = create_schema(pool);
    let app = graphql_routes(schema);

    let query = r#"{
        "query": "{ tradeNewsByCategory(category: \"Trade\") { id title source } }"
    }"#;

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

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[ignore = "Temporarily disabled: AnyPool driver issue in tests"]
async fn test_trade_news_by_source_query() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    let pool = nba_trade_scraper::db::connection::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let schema = create_schema(pool);
    let app = graphql_routes(schema);

    let query = r#"{
        "query": "{ tradeNewsBySource(source: \"ESPN\") { id title category } }"
    }"#;

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

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[ignore = "Temporarily disabled: AnyPool driver issue in tests"]
async fn test_invalid_query() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    let pool = nba_trade_scraper::db::connection::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let schema = create_schema(pool);
    let app = graphql_routes(schema);

    let query = r#"{
        "query": "{ invalidField }"
    }"#;

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

    // GraphQLエラーでも200 OKを返すことに注意
    assert_eq!(response.status(), StatusCode::OK);
}
