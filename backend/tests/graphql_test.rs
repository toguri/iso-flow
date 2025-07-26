use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use nba_trade_scraper::graphql::{create_schema, graphql_routes};
use tower::ServiceExt;

#[tokio::test]
#[ignore = "Requires PostgreSQL database connection"]
async fn test_graphql_playground() {
    let pool = match nba_trade_scraper::db::connection::create_pool().await {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping test: PostgreSQL database connection required");
            return;
        }
    };
    sqlx::migrate!("./migrations_postgres").run(&pool).await.unwrap();

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
#[ignore = "Requires PostgreSQL database connection"]
async fn test_trade_news_query() {
    let pool = match nba_trade_scraper::db::connection::create_pool().await {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping test: PostgreSQL database connection required");
            return;
        }
    };
    sqlx::migrate!("./migrations_postgres").run(&pool).await.unwrap();

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
#[ignore = "Requires PostgreSQL database connection"]
async fn test_trade_news_by_category_query() {
    let pool = match nba_trade_scraper::db::connection::create_pool().await {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping test: PostgreSQL database connection required");
            return;
        }
    };
    sqlx::migrate!("./migrations_postgres").run(&pool).await.unwrap();

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
#[ignore = "Requires PostgreSQL database connection"]
async fn test_trade_news_by_source_query() {
    let pool = match nba_trade_scraper::db::connection::create_pool().await {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping test: PostgreSQL database connection required");
            return;
        }
    };
    sqlx::migrate!("./migrations_postgres").run(&pool).await.unwrap();

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
#[ignore = "Requires PostgreSQL database connection"]
async fn test_invalid_query() {
    let pool = match nba_trade_scraper::db::connection::create_pool().await {
        Ok(pool) => pool,
        Err(_) => {
            eprintln!("Skipping test: PostgreSQL database connection required");
            return;
        }
    };
    sqlx::migrate!("./migrations_postgres").run(&pool).await.unwrap();

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
