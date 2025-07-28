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

    #[test]
    fn test_pg_news_repository_creation() {
        // PgNewsRepositoryの作成テスト（実際のプールは作成しない）
        // このテストは構造体が正しく定義されていることを確認
        use std::mem::size_of;
        assert!(size_of::<PgNewsRepository>() > 0);
    }
}
