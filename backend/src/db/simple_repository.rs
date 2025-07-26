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
    use crate::db::repository::{mock::MockNewsRepository, NewsRepository};
    use crate::scraper::{NewsItem, NewsSource};
    use chrono::Utc;

    #[tokio::test]
    async fn test_repository_with_mock() {
        // モックリポジトリを作成
        let mock_repo = MockNewsRepository::new();

        // テストデータを作成
        let test_item = NewsItem {
            id: "test-001".to_string(),
            title: "LeBron James traded to Lakers".to_string(),
            description: Some("Big trade news".to_string()),
            link: "https://example.com/news/1".to_string(),
            source: NewsSource::ESPN,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        };

        // ニュースを保存
        mock_repo.save_news(vec![test_item.clone()]).await.unwrap();

        // 保存されたニュースを確認
        let all_news = mock_repo.get_all_news().await.unwrap();
        assert_eq!(all_news.len(), 1);
        assert_eq!(all_news[0].id, "test-001");
        assert_eq!(all_news[0].title, "LeBron James traded to Lakers");
    }

    #[tokio::test]
    async fn test_repository_category_filter() {
        // テストデータを複数作成
        let items = vec![
            NewsItem {
                id: "test-002".to_string(),
                title: "Trade News 1".to_string(),
                description: None,
                link: "https://example.com/2".to_string(),
                source: NewsSource::ESPN,
                published_at: Utc::now(),
                category: "Trade".to_string(),
            },
            NewsItem {
                id: "test-003".to_string(),
                title: "Signing News 1".to_string(),
                description: None,
                link: "https://example.com/3".to_string(),
                source: NewsSource::RealGM,
                published_at: Utc::now(),
                category: "Signing".to_string(),
            },
        ];

        let mock_repo = MockNewsRepository::with_news(items);

        // カテゴリー別にフィルタリング
        let trade_news = mock_repo.get_news_by_category("Trade").await.unwrap();
        assert_eq!(trade_news.len(), 1);
        assert_eq!(trade_news[0].category, "Trade");

        let signing_news = mock_repo.get_news_by_category("Signing").await.unwrap();
        assert_eq!(signing_news.len(), 1);
        assert_eq!(signing_news[0].category, "Signing");
    }
}
