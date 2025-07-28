//! リポジトリ実装
//!
//! データベースアクセスを抽象化するためのトレイトを定義します。

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;

use crate::scraper::NewsItem;

/// PostgreSQL実装
pub struct PgNewsRepository {
    pool: PgPool,
}

impl PgNewsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_news(&self) -> Result<Vec<NewsItem>> {
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

    pub async fn get_news_by_category(&self, category: &str) -> Result<Vec<NewsItem>> {
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

    pub async fn get_news_by_source(&self, source: &str) -> Result<Vec<NewsItem>> {
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

    pub async fn save_news(&self, items: Vec<NewsItem>) -> Result<()> {
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

    pub async fn get_recent_news(&self, since: DateTime<Utc>) -> Result<Vec<NewsItem>> {
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


#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPool;
    use crate::scraper::{NewsItem, NewsSource};
    use chrono::Utc;

    async fn setup_test_db() -> Option<PgPool> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://test_user:test_password@localhost:5433/test_iso_flow".to_string());
        
        match PgPool::connect(&database_url).await {
            Ok(pool) => Some(pool),
            Err(_) => {
                eprintln!("Skipping test: PostgreSQL database required");
                None
            }
        }
    }

    #[test]
    fn test_pg_news_repository_creation() {
        // PgNewsRepositoryの作成テスト（実際のプールは作成しない）
        // このテストは構造体が正しく定義されていることを確認
        use std::mem::size_of;
        assert!(size_of::<PgNewsRepository>() > 0);
    }

    #[tokio::test]
    async fn test_save_and_retrieve_news() {
        let Some(pool) = setup_test_db().await else { return; };
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
        let Some(pool) = setup_test_db().await else { return; };
        let repo = PgNewsRepository::new(pool.clone());
        
        let trade_item = NewsItem {
            id: format!("trade-{}", Utc::now().timestamp_nanos()),
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
        let Some(pool) = setup_test_db().await else { return; };
        let repo = PgNewsRepository::new(pool.clone());
        
        let espn_item = NewsItem {
            id: format!("espn-{}", Utc::now().timestamp_nanos()),
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
        let Some(pool) = setup_test_db().await else { return; };
        let repo = PgNewsRepository::new(pool.clone());
        
        let mut news_items = vec![];
        for i in 0..3 {
            news_items.push(NewsItem {
                id: format!("recent-{}-{}", i, Utc::now().timestamp_nanos()),
                title: format!("Recent News {}", i),
                description: Some(format!("Description {}", i)),
                link: format!("https://example.com/recent/{}", i),
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
