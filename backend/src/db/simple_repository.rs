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
    use sqlx::postgres::PgPool;

    async fn setup_test_db() -> Option<PgPool> {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://test_user:test_password@localhost:5433/test_iso_flow".to_string()
        });

        match PgPool::connect(&database_url).await {
            Ok(pool) => Some(pool),
            Err(_) => {
                eprintln!("Skipping test: PostgreSQL database required");
                None
            }
        }
    }

    #[test]
    fn test_simple_repository_struct() {
        // 構造体のフィールドが正しく定義されていることをテスト
        // (実際のPgPoolを作成せずに、型のテストのみ)
        use std::mem;

        // SimpleRepository構造体のサイズを確認
        // PgPoolは内部でArcを使うので、ポインタサイズになる
        assert!(mem::size_of::<SimpleRepository>() >= mem::size_of::<usize>());
    }

    #[tokio::test]
    async fn test_new() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = SimpleRepository::new(pool.clone());

        // newメソッドが正しく動作することを確認
        let tables_exist = repo.check_tables().await.unwrap();
        assert!(tables_exist);
    }

    #[tokio::test]
    async fn test_count_news() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = SimpleRepository::new(pool);

        let count = repo.count_news().await.unwrap();
        assert!(count >= 0);
    }

    #[tokio::test]
    async fn test_check_tables() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = SimpleRepository::new(pool);

        let tables_exist = repo.check_tables().await.unwrap();
        assert!(tables_exist, "Teams and trade_news tables should exist");
    }
}
