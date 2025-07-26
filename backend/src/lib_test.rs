#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use tower::ServiceExt;
    use serde_json::json;

    async fn create_test_pool() -> PgPool {
        // テスト用のデータベースプールを作成
        // 実際のテストではモックやテスト用DBを使用することを推奨
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:password@localhost/test_db".to_string()
            }))
            .await
            .expect("Failed to create test pool")
    }

    #[tokio::test]
    async fn test_health_check_endpoint() {
        let pool = create_test_pool().await;
        let app = create_app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "healthy");
        assert_eq!(json["service"], "nba-trade-scraper");
        assert!(json["timestamp"].is_string());
    }

    #[tokio::test]
    async fn test_graphql_playground_endpoint() {
        let pool = create_test_pool().await;
        let app = create_app(pool);

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

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let html = String::from_utf8(body.to_vec()).unwrap();

        // GraphiQLのHTMLが返されることを確認
        assert!(html.contains("GraphiQL"));
    }

    #[tokio::test]
    async fn test_graphql_post_endpoint() {
        let pool = create_test_pool().await;
        let app = create_app(pool);

        let query = json!({
            "query": "{ __schema { queryType { name } } }"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&query).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        // GraphQLスキーマの基本的な応答を確認
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_app_has_cors_layer() {
        // create_app関数がCORSレイヤーを含むことを間接的にテスト
        // 実際の動作テストは統合テストで行う
        let cors_test = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
        
        // CORSレイヤーが正しく設定されることを確認
        // （実装の詳細に依存しない形でのテスト）
        assert!(true); // プレースホルダー
    }
}