//! データベース接続とマイグレーション管理
//!
//! このモジュールは、PostgreSQLデータベースへの接続プールの作成と
//! マイグレーションの実行を担当します。

pub mod connection;
pub mod models;
pub mod simple_repository;

// Note: create_pool is now in the connection module and returns PgPool
