/// シンプルなリポジトリ実装（SQLxマクロを使わない）
use anyhow::Result;
use sqlx::SqlitePool;

pub struct SimpleRepository {
    pool: SqlitePool,
}

impl SimpleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// ニュースの件数を取得
    pub async fn count_news(&self) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM trade_news")
            .fetch_one(&self.pool)
            .await?;
        Ok(result)
    }

    /// テーブルが存在することを確認
    pub async fn check_tables(&self) -> Result<bool> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('teams', 'trade_news')"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result == 2)
    }
}