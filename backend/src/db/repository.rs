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
    use crate::scraper::{NewsItem, NewsSource};
    use chrono::Utc;
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
    fn test_pg_news_repository_creation() {
        // PgNewsRepositoryの作成テスト（実際のプールは作成しない）
        // このテストは構造体が正しく定義されていることを確認
        use std::mem::size_of;
        assert!(size_of::<PgNewsRepository>() > 0);
    }

    #[tokio::test]
    async fn test_save_and_retrieve_news() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let timestamp = Utc::now().timestamp_nanos_opt().unwrap();
        let test_item = NewsItem {
            id: format!("test-{}", timestamp),
            title: "Test News".to_string(),
            description: Some("Test description".to_string()),
            link: format!("https://example.com/test-{}", timestamp),
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

    #[tokio::test]
    async fn test_save_news_with_empty_list() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        // 空のリストを保存しても問題ないことを確認
        let result = repo.save_news(vec![]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_news_with_duplicate_id() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let test_item = NewsItem {
            id: format!("duplicate-{}", Utc::now().timestamp_nanos_opt().unwrap()),
            title: "Original News".to_string(),
            description: Some("Original description".to_string()),
            link: "https://example.com/original".to_string(),
            source: NewsSource::ESPN,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        };

        // 最初の保存
        repo.save_news(vec![test_item.clone()]).await.unwrap();

        // 同じIDで異なる内容のアイテムを保存（ON CONFLICT DO NOTHINGにより無視される）
        let duplicate_item = NewsItem {
            id: test_item.id.clone(),
            title: "Modified News".to_string(),
            description: Some("Modified description".to_string()),
            link: "https://example.com/modified".to_string(),
            source: NewsSource::RealGM,
            published_at: Utc::now(),
            category: "Rumor".to_string(),
        };

        let result = repo.save_news(vec![duplicate_item]).await;
        assert!(result.is_ok());

        // 元のデータが保持されていることを確認
        let all_news = repo.get_all_news().await.unwrap();
        let saved_item = all_news
            .iter()
            .find(|item| item.id == test_item.id)
            .unwrap();
        assert_eq!(saved_item.title, "Original News");
        assert_eq!(saved_item.source, NewsSource::ESPN);

        sqlx::query("DELETE FROM trade_news WHERE id = $1")
            .bind(&test_item.id)
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_news_by_category_empty_result() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool);

        // 存在しないカテゴリで検索
        let result = repo
            .get_news_by_category("NonExistentCategory")
            .await
            .unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_get_news_by_source_empty_result() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool);

        // 存在しないソースで検索
        let result = repo.get_news_by_source("NonExistentSource").await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_get_recent_news_with_future_date() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool);

        // 未来の日付で検索（結果は空になるはず）
        let future_date = Utc::now() + chrono::Duration::hours(24);
        let result = repo.get_recent_news(future_date).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_save_and_retrieve_news_with_null_description() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let test_item = NewsItem {
            id: format!("null-desc-{}", Utc::now().timestamp_nanos_opt().unwrap()),
            title: "News without description".to_string(),
            description: None,
            link: "https://example.com/no-desc".to_string(),
            source: NewsSource::ESPN,
            published_at: Utc::now(),
            category: "Trade".to_string(),
        };

        repo.save_news(vec![test_item.clone()]).await.unwrap();

        let all_news = repo.get_all_news().await.unwrap();
        let saved_item = all_news
            .iter()
            .find(|item| item.id == test_item.id)
            .unwrap();
        assert_eq!(saved_item.description, None);

        sqlx::query("DELETE FROM trade_news WHERE id = $1")
            .bind(&test_item.id)
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_multiple_sources_and_categories() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let sources = vec![NewsSource::ESPN, NewsSource::RealGM, NewsSource::HoopsHype];
        let categories = vec!["Trade", "Rumor", "Analysis"];
        let mut all_items = vec![];

        // 各ソースとカテゴリの組み合わせでニュースを作成
        for (i, (source, category)) in sources.iter().zip(categories.iter()).enumerate() {
            let item = NewsItem {
                id: format!("multi-{}-{}", i, Utc::now().timestamp_nanos_opt().unwrap()),
                title: format!("{} {} News", source.to_string(), category),
                description: Some(format!(
                    "Description for {} {}",
                    source.to_string(),
                    category
                )),
                link: format!(
                    "https://example.com/{}/{}",
                    source.to_string().to_lowercase(),
                    category.to_lowercase()
                ),
                source: source.clone(),
                published_at: Utc::now(),
                category: category.to_string(),
            };
            all_items.push(item);
        }

        repo.save_news(all_items.clone()).await.unwrap();

        // 各カテゴリで検索
        for category in &categories {
            let news_by_category = repo.get_news_by_category(category).await.unwrap();
            assert!(!news_by_category.is_empty());
            assert!(news_by_category
                .iter()
                .all(|item| &item.category == category));
        }

        // 各ソースで検索
        for source in &sources {
            let news_by_source = repo.get_news_by_source(&source.to_string()).await.unwrap();
            assert!(!news_by_source.is_empty());
            assert!(news_by_source.iter().all(|item| item.source == *source));
        }

        // クリーンアップ
        for item in &all_items {
            sqlx::query("DELETE FROM trade_news WHERE id = $1")
                .bind(&item.id)
                .execute(&pool)
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn test_ordering_in_get_all_news() {
        let Some(pool) = setup_test_db().await else {
            return;
        };
        let repo = PgNewsRepository::new(pool.clone());

        let base_time = Utc::now();
        let mut news_items = vec![];

        // 異なる時刻のニュースを作成
        for i in 0..5 {
            news_items.push(NewsItem {
                id: format!("order-{}-{}", i, base_time.timestamp_nanos_opt().unwrap()),
                title: format!("News {}", i),
                description: Some(format!("Description {}", i)),
                link: format!("https://example.com/news/{}", i),
                source: NewsSource::ESPN,
                published_at: base_time - chrono::Duration::hours(i),
                category: "Trade".to_string(),
            });
        }

        repo.save_news(news_items.clone()).await.unwrap();

        let all_news = repo.get_all_news().await.unwrap();

        // published_at降順で並んでいることを確認
        let mut prev_time = None;
        for item in &all_news {
            if item.id.starts_with("order-") {
                if let Some(prev) = prev_time {
                    assert!(item.published_at <= prev);
                }
                prev_time = Some(item.published_at);
            }
        }

        // クリーンアップ
        for item in &news_items {
            sqlx::query("DELETE FROM trade_news WHERE id = $1")
                .bind(&item.id)
                .execute(&pool)
                .await
                .unwrap();
        }
    }
}
