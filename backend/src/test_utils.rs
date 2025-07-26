//! テスト用ユーティリティ
//!
//! モックを使ったテスト用のヘルパー関数を提供します。

use std::sync::Arc;
use axum::Router;
use async_graphql::{EmptySubscription, Schema};
use crate::{
    db::repository::{mock::MockNewsRepository, NewsRepository},
    graphql::{create_schema_with_repository, Query, Mutation},
};
use tower_http::cors::{Any, CorsLayer};

/// モックリポジトリを使用してテスト用アプリケーションを作成
pub fn create_test_app(repository: Arc<dyn NewsRepository>) -> Router {
    let schema = create_schema_with_repository(repository);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", axum::routing::get(graphiql).post(graphql_handler))
        .route("/health", axum::routing::get(health_check))
        .layer(cors)
        .layer(axum::extract::Extension(schema))
}

/// テスト用のモックアプリケーションを作成（空のデータ）
pub fn create_mock_app() -> Router {
    let mock_repo = Arc::new(MockNewsRepository::new());
    create_test_app(mock_repo)
}

/// テスト用のモックアプリケーションを作成（初期データ付き）
pub fn create_mock_app_with_data(news: Vec<crate::scraper::NewsItem>) -> Router {
    let mock_repo = Arc::new(MockNewsRepository::with_news(news));
    create_test_app(mock_repo)
}

async fn graphql_handler(
    schema: axum::extract::Extension<Schema<Query, Mutation, EmptySubscription>>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> axum::response::Html<String> {
    axum::response::Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/")
            .finish(),
    )
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "nba-trade-scraper",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}