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
//!
//! #[tokio::main]
//! async fn main() {
//!     // GraphQLスキーマの作成
//!     let schema = create_schema();
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

/// RSSスクレイピング機能
pub mod scraper;
