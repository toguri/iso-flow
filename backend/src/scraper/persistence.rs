use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;

use crate::scraper::models::NewsItem;

/// スクレイピングしたデータをデータベースに保存
pub async fn save_news_items(pool: &SqlitePool, items: Vec<NewsItem>) -> Result<usize> {
    let mut saved_count = 0;

    for item in items {
        // 既存チェック
        let exists = sqlx::query!(
            "SELECT COUNT(*) as count FROM trade_news WHERE external_id = ?1",
            item.id
        )
        .fetch_one(pool)
        .await?;

        if exists.count == 0 {
            // 新規保存
            let now = Utc::now().to_rfc3339();
            let published_at = item.published_at.to_rfc3339();

            sqlx::query!(
                r#"
                INSERT INTO trade_news (
                    external_id, title, description, source_name, source_url,
                    category, published_at, scraped_at, created_at, updated_at
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                "#,
                item.id,
                item.title,
                item.description,
                item.source,
                item.link,
                item.category,
                published_at,
                now,
                now,
                now
            )
            .execute(pool)
            .await?;

            saved_count += 1;
            tracing::info!("Saved new trade news: {}", item.title);
        }
    }

    Ok(saved_count)
}
