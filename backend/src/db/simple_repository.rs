/// シンプルなリポジトリ実装（SQLxマクロを使わない）
use anyhow::Result;
use sqlx::postgres::PgPool;

pub struct SimpleRepository {
    pool: PgPool,
}

impl SimpleRepository {
    pub fn new(pool: PgPool) -> Self {
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
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public' AND table_name IN ('teams', 'trade_news')"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result == 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires PostgreSQL database"]
    async fn test_simple_repository() {
        // PostgreSQLデータベースで初期化
        // テスト環境ではPostgreSQLのセットアップが必要
        let pool = crate::db::connection::create_pool().await.unwrap();

        // マイグレーションを実行
        sqlx::migrate!("./migrations_postgres")
            .run(&pool)
            .await
            .unwrap();

        let repo = SimpleRepository::new(pool.clone());

        // テーブルの存在確認
        let has_tables = repo.check_tables().await.unwrap();
        assert!(has_tables);

        // ニュース件数の確認（初期状態では0件）
        let count = repo.count_news().await.unwrap();
        assert_eq!(count, 0);
    }
}
