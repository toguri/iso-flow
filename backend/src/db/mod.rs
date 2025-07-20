use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::env;

pub mod models;

pub type DbPool = Pool<Sqlite>;

pub async fn create_pool() -> Result<DbPool> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    // SQLiteの接続プールを作成
    let pool = SqlitePool::connect(&database_url).await?;

    // マイグレーションを実行
    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &DbPool) -> Result<()> {
    sqlx::migrate!("migrations").run(pool).await?;

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
