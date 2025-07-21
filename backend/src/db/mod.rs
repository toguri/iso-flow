//! データベース接続とマイグレーション管理
//!
//! このモジュールは、SQLiteデータベースへの接続プールの作成と
//! マイグレーションの実行を担当します。

use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::env;

pub mod models;

/// データベース接続プールの型エイリアス
pub type DbPool = Pool<Sqlite>;

/// データベース接続プールを作成し、マイグレーションを実行します
///
/// # 環境変数
///
/// - `DATABASE_URL`: データベースのURL（デフォルト: `sqlite::memory:`）
///
/// # エラー
///
/// データベース接続の失敗またはマイグレーションの失敗時にエラーを返します
///
/// # 例
///
/// ```no_run
/// # use nba_trade_scraper::db;
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let pool = db::create_pool().await?;
/// # Ok(())
/// # }
/// ```
pub async fn create_pool() -> Result<DbPool> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    // SQLiteの接続プールを作成
    let pool = SqlitePool::connect(&database_url).await?;

    // マイグレーションを実行
    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &DbPool) -> Result<()> {
    // Use absolute path from CARGO_MANIFEST_DIR for CI compatibility
    sqlx::migrate!().run(pool).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        env::set_var("DATABASE_URL", "sqlite::memory:");
        let pool = create_pool().await;
        assert!(pool.is_ok());
    }
}
