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
    use crate::scraper::{NewsItem, NewsSource};

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
}
