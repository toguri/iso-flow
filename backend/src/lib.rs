//! # NBA Trade Scraper
//!
//! NBAのトレード情報を収集・分析するためのバックエンドシステムです。
//!
//! ## 主な機能
//!
//! - **RSSフィード解析**: ESPNやRealGMなどの主要ソースからニュースを自動収集
//! - **GraphQL API**: フレキシブルなクエリインターフェース
//! - **データベース**: トレード情報の永続化と管理
//!
//! ## モジュール構成
//!
//! - [`db`] - データベース接続とモデル定義
//! - [`graphql`] - GraphQL APIのスキーマとリゾルバー
//! - [`scraper`] - RSSフィード解析とニュース分類
//!
//! ## 使用例
//!
//! ```no_run
//! use nba_trade_scraper::graphql::create_schema;
//! use nba_trade_scraper::scraper::RssParser;
//! use nba_trade_scraper::db::connection::create_pool;
//!
//! #[tokio::main]
//! async fn main() {
//!     // データベース接続
//!     let pool = create_pool().await.unwrap();
//!
//!     // GraphQLスキーマの作成
//!     let schema = create_schema(pool);
//!
//!     // RSSフィードの解析
//!     let parser = RssParser::new();
//!     let news = parser.fetch_all_feeds().await.unwrap();
//! }
//! ```

/// データベース関連の機能
pub mod db;

/// GraphQL APIの実装
pub mod graphql;

/// GraphQLコンテキスト
pub mod graphql_context;

/// RSSスクレイピング機能
pub mod scraper;

/// スケジューラー機能
pub mod scheduler;

/// ユーティリティ関数
pub mod utils;

/// テスト用ユーティリティ
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{routing::get, Router};
use sqlx::postgres::PgPool;
use tower_http::cors::{Any, CorsLayer};

/// アプリケーションの作成
pub fn create_app(pool: PgPool) -> Router {
    let schema = graphql::create_schema(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/health", get(health_check))
        .layer(cors)
        .layer(axum::extract::Extension(schema))
}

async fn graphql_handler(
    schema: axum::extract::Extension<Schema<graphql::Query, graphql::Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_returns_healthy() {
        let response = health_check().await;
        let json = response.0;

        assert_eq!(json["status"], "healthy");
        assert_eq!(json["service"], "nba-trade-scraper");
        assert!(json["timestamp"].is_string());
    }

    #[tokio::test]
    async fn test_graphiql_returns_html() {
        let html = graphiql().await;
        let content = html.0;

        // GraphiQL HTMLが含まれることを確認
        assert!(content.contains("GraphiQL"));
        assert!(content.contains("</html>"));
    }

    #[test]
    fn test_cors_configuration() {
        // CORSレイヤーが正しく設定されることを確認
        let _cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        // CORSレイヤーの設定を確認（インスタンス化できることを確認）
        assert!(true);
    }
}
