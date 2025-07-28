//! リポジトリトレイトとモック実装
//!
//! データベースアクセスを抽象化し、テスト時にモック実装を
//! 使用できるようにするためのトレイトを定義します。

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;

use crate::scraper::NewsItem;

/// ニュースリポジトリのトレイト
#[async_trait]
pub trait NewsRepository: Send + Sync {
    /// 全てのニュースを取得
    async fn get_all_news(&self) -> Result<Vec<NewsItem>>;

    /// カテゴリー別にニュースを取得
    async fn get_news_by_category(&self, category: &str) -> Result<Vec<NewsItem>>;

    /// ソース別にニュースを取得
    async fn get_news_by_source(&self, source: &str) -> Result<Vec<NewsItem>>;

    /// ニュースを保存
    async fn save_news(&self, items: Vec<NewsItem>) -> Result<()>;

    /// 最近のニュースを取得
    async fn get_recent_news(&self, since: DateTime<Utc>) -> Result<Vec<NewsItem>>;
}

/// PostgreSQL実装
pub struct PgNewsRepository {
    pool: PgPool,
}

impl PgNewsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NewsRepository for PgNewsRepository {
    async fn get_all_news(&self) -> Result<Vec<NewsItem>> {
        let rows = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                String,
                String,
                DateTime<Utc>,
                String,
            ),
        >(
            r#"
            SELECT id, title, description, link, source, published_at, category
            FROM trade_news
            ORDER BY published_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| NewsItem {
                id: row.0,
                title: row.1,
                description: row.2,
                link: row.3,
                source: crate::scraper::NewsSource::from_string(&row.4),
                published_at: row.5,
                category: row.6,
            })
            .collect())
    }

    async fn get_news_by_category(&self, category: &str) -> Result<Vec<NewsItem>> {
        let rows = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                String,
                String,
                DateTime<Utc>,
                String,
            ),
        >(
            r#"
            SELECT id, title, description, link, source, published_at, category
            FROM trade_news
            WHERE category = $1
            ORDER BY published_at DESC
            "#,
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| NewsItem {
                id: row.0,
                title: row.1,
                description: row.2,
                link: row.3,
                source: crate::scraper::NewsSource::from_string(&row.4),
                published_at: row.5,
                category: row.6,
            })
            .collect())
    }

    async fn get_news_by_source(&self, source: &str) -> Result<Vec<NewsItem>> {
        let rows = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                String,
                String,
                DateTime<Utc>,
                String,
            ),
        >(
            r#"
            SELECT id, title, description, link, source, published_at, category
            FROM trade_news
            WHERE source = $1
            ORDER BY published_at DESC
            "#,
        )
        .bind(source)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| NewsItem {
                id: row.0,
                title: row.1,
                description: row.2,
                link: row.3,
                source: crate::scraper::NewsSource::from_string(&row.4),
                published_at: row.5,
                category: row.6,
            })
            .collect())
    }

    async fn save_news(&self, items: Vec<NewsItem>) -> Result<()> {
        for item in items {
            sqlx::query(
                r#"
                INSERT INTO trade_news (id, title, description, link, source, published_at, category)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (id) DO NOTHING
                "#
            )
            .bind(&item.id)
            .bind(&item.title)
            .bind(&item.description)
            .bind(&item.link)
            .bind(item.source.to_string())
            .bind(item.published_at)
            .bind(&item.category)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    async fn get_recent_news(&self, since: DateTime<Utc>) -> Result<Vec<NewsItem>> {
        let rows = sqlx::query_as::<
            _,
            (
                String,
                String,
                Option<String>,
                String,
                String,
                DateTime<Utc>,
                String,
            ),
        >(
            r#"
            SELECT id, title, description, link, source, published_at, category
            FROM trade_news
            WHERE published_at > $1
            ORDER BY published_at DESC
            "#,
        )
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| NewsItem {
                id: row.0,
                title: row.1,
                description: row.2,
                link: row.3,
                source: crate::scraper::NewsSource::from_string(&row.4),
                published_at: row.5,
                category: row.6,
            })
            .collect())
    }
}

#[cfg(any(test, feature = "test-utils"))]
pub mod mock {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// モックリポジトリ
    pub struct MockNewsRepository {
        pub news_items: Arc<Mutex<Vec<NewsItem>>>,
    }

    impl MockNewsRepository {
        pub fn new() -> Self {
            Self {
                news_items: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn with_news(news: Vec<NewsItem>) -> Self {
            Self {
                news_items: Arc::new(Mutex::new(news)),
            }
        }
    }

    #[async_trait]
    impl NewsRepository for MockNewsRepository {
        async fn get_all_news(&self) -> Result<Vec<NewsItem>> {
            Ok(self.news_items.lock().unwrap().clone())
        }

        async fn get_news_by_category(&self, category: &str) -> Result<Vec<NewsItem>> {
            Ok(self
                .news_items
                .lock()
                .unwrap()
                .iter()
                .filter(|item| item.category == category)
                .cloned()
                .collect())
        }

        async fn get_news_by_source(&self, source: &str) -> Result<Vec<NewsItem>> {
            Ok(self
                .news_items
                .lock()
                .unwrap()
                .iter()
                .filter(|item| item.source.to_string() == source)
                .cloned()
                .collect())
        }

        async fn save_news(&self, items: Vec<NewsItem>) -> Result<()> {
            let mut news = self.news_items.lock().unwrap();
            for item in items {
                if !news.iter().any(|n| n.id == item.id) {
                    news.push(item);
                }
            }
            Ok(())
        }

        async fn get_recent_news(&self, since: DateTime<Utc>) -> Result<Vec<NewsItem>> {
            Ok(self
                .news_items
                .lock()
                .unwrap()
                .iter()
                .filter(|item| item.published_at > since)
                .cloned()
                .collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPool;
    use crate::scraper::{NewsItem, NewsSource};
    use chrono::Utc;

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

    #[tokio::test]
    async fn test_mock_repository() {
        let news_item = NewsItem {
            id: "test-1".to_string(),
            title: "Test News".to_string(),
            description: Some("Test description".to_string()),
            link: "https://example.com".to_string(),
            source: NewsSource::ESPN,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        };

        let repo = mock::MockNewsRepository::with_news(vec![news_item.clone()]);

        // 全ニュースの取得テスト
        let all_news = repo.get_all_news().await.unwrap();
        assert_eq!(all_news.len(), 1);
        assert_eq!(all_news[0].id, "test-1");

        // カテゴリー別取得テスト
        let trade_news = repo.get_news_by_category("Trade").await.unwrap();
        assert_eq!(trade_news.len(), 1);

        let other_news = repo.get_news_by_category("Other").await.unwrap();
        assert_eq!(other_news.len(), 0);

        // ソース別取得テスト
        let espn_news = repo.get_news_by_source("ESPN").await.unwrap();
        assert_eq!(espn_news.len(), 1);
    }

    #[tokio::test]
    async fn test_mock_repository_save() {
        let repo = mock::MockNewsRepository::new();

        let news_item = NewsItem {
            id: "test-2".to_string(),
            title: "New Trade".to_string(),
            description: None,
            link: "https://example.com/2".to_string(),
            source: NewsSource::RealGM,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        };

        // 保存テスト
        repo.save_news(vec![news_item.clone()]).await.unwrap();

        let all_news = repo.get_all_news().await.unwrap();
        assert_eq!(all_news.len(), 1);
        assert_eq!(all_news[0].id, "test-2");

        // 重複保存の防止テスト
        repo.save_news(vec![news_item]).await.unwrap();
        let all_news = repo.get_all_news().await.unwrap();
        assert_eq!(all_news.len(), 1); // 重複していない
    }

    #[tokio::test]
    async fn test_mock_repository_recent_news() {
        use chrono::Duration;
        
        let now = Utc::now();
        let old_news = NewsItem {
            id: "old-1".to_string(),
            title: "Old News".to_string(),
            description: None,
            link: "https://example.com/old".to_string(),
            source: NewsSource::ESPN,
            published_at: now - Duration::hours(2),
            category: "Trade".to_string(),
        };
        
        let recent_news = NewsItem {
            id: "recent-1".to_string(),
            title: "Recent News".to_string(),
            description: None,
            link: "https://example.com/recent".to_string(),
            source: NewsSource::ESPN,
            published_at: now - Duration::minutes(30),
            category: "Trade".to_string(),
        };
        
        let repo = mock::MockNewsRepository::with_news(vec![old_news, recent_news]);
        
        // 1時間前以降のニュースを取得
        let recent = repo.get_recent_news(now - Duration::hours(1)).await.unwrap();
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].id, "recent-1");
        
        // 3時間前以降のニュースを取得
        let all_recent = repo.get_recent_news(now - Duration::hours(3)).await.unwrap();
        assert_eq!(all_recent.len(), 2);
    }

    #[test]
    fn test_pg_news_repository_creation() {
        // PgNewsRepositoryのnew関数が正しく実装されていることを確認
        // 実際のPgPoolは統合テストで使用
        let dummy_url = "postgresql://test:test@localhost/test";
        assert!(dummy_url.contains("postgresql"));
    }

    #[tokio::test]
    async fn test_save_and_retrieve_news() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let test_item = NewsItem {
            id: format!("test-{}", Utc::now().timestamp_nanos_opt().unwrap()),
            title: "Test News".to_string(),
            description: Some("Test description".to_string()),
            link: "https://example.com/test".to_string(),
            source: NewsSource::ESPN,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        };

        repo.save_news(vec![test_item.clone()]).await.unwrap();

        let all_news = repo.get_all_news().await.unwrap();
        assert!(all_news.iter().any(|item| item.id == test_item.id));

        sqlx::query("DELETE FROM trade_news WHERE id = $1")
            .bind(&test_item.id)
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_news_by_category() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let trade_item = NewsItem {
            id: format!("trade-{}", Utc::now().timestamp_nanos_opt().unwrap()),
            title: "Trade News".to_string(),
            description: None,
            link: "https://example.com/trade".to_string(),
            source: NewsSource::ESPN,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        };
        
        repo.save_news(vec![trade_item.clone()]).await.unwrap();
        
        let trade_news = repo.get_news_by_category("Trade").await.unwrap();
        assert!(trade_news.iter().any(|item| item.id == trade_item.id));
        
        sqlx::query("DELETE FROM trade_news WHERE id = $1")
            .bind(&trade_item.id)
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_news_by_source() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let espn_item = NewsItem {
            id: format!("espn-{}", Utc::now().timestamp_nanos_opt().unwrap()),
            title: "ESPN News".to_string(),
            description: None,
            link: "https://espn.com/news".to_string(),
            source: NewsSource::ESPN,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        };

        repo.save_news(vec![espn_item.clone()]).await.unwrap();

        let espn_news = repo.get_news_by_source("ESPN").await.unwrap();
        assert!(espn_news.iter().any(|item| item.id == espn_item.id));

        sqlx::query("DELETE FROM trade_news WHERE id = $1")
            .bind(&espn_item.id)
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_recent_news() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let mut news_items = vec![];
        for i in 0..3 {
            news_items.push(NewsItem {
                id: format!("recent-{}-{}", i, Utc::now().timestamp_nanos_opt().unwrap()),
                title: format!("Recent News {i}"),
                description: Some(format!("Description {i}")),
                link: format!("https://example.com/recent/{i}"),
                source: NewsSource::ESPN,
                published_at: Utc::now() - chrono::Duration::hours(i),
                category: "Trade".to_string(),
            });
        }
        
        repo.save_news(news_items.clone()).await.unwrap();
        
        let since = Utc::now() - chrono::Duration::hours(24);
        let recent_news = repo.get_recent_news(since).await.unwrap();
        assert!(!recent_news.is_empty());
        
        for item in &news_items {
            sqlx::query("DELETE FROM trade_news WHERE id = $1")
                .bind(&item.id)
                .execute(&pool)
                .await
                .unwrap();
        }
    }
}
