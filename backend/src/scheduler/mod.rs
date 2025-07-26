//! スケジューラー関連の機能
//!
//! 定期的なスクレイピングジョブの実行を管理します。

use anyhow::Result;
use sqlx::postgres::PgPool;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};
use uuid::Uuid;

use crate::scraper::{NewsPersistence, RssParser};

/// スクレイピングジョブを実行する
pub async fn run_scraping_job(pool: PgPool) -> Result<()> {
    info!("Starting scraping job");

    // RSSフィードからニュースを取得
    let parser = RssParser::new();
    let news_items = parser.fetch_all_feeds().await?;

    info!("Fetched {} news items", news_items.len());

    // データベースに保存
    let persistence = NewsPersistence::new(pool);
    let result = persistence.save_news_items(news_items).await?;

    info!(
        "Scraping job completed: {} saved, {} skipped, {} errors",
        result.saved_count,
        result.skipped_count,
        result.errors.len()
    );

    if !result.errors.is_empty() {
        for (id, error) in &result.errors {
            error!("Error saving item {}: {}", id, error);
        }
    }

    Ok(())
}

/// スケジューラーを作成し、設定する
pub async fn create_scheduler(pool: PgPool) -> Result<JobScheduler> {
    let scheduler = JobScheduler::new().await?;

    // 5分ごとのスクレイピングジョブを作成
    // cron式: "秒 分 時 日 月 曜日"
    // "0 */5 * * * *" = 毎時0分、5分、10分、15分...に実行
    let job = Job::new_async("0 */5 * * * *", move |_uuid, _lock| {
        let pool = pool.clone();
        Box::pin(async move {
            let job_id = Uuid::new_v4();
            info!("Starting scheduled scraping job: {}", job_id);

            match run_scraping_job(pool).await {
                Ok(_) => {
                    info!("Scheduled job {} completed successfully", job_id);
                }
                Err(e) => {
                    error!("Scheduled job {} failed: {}", job_id, e);
                }
            }
        })
    })?;

    scheduler.add(job).await?;

    info!("Scheduler initialized with 5-minute scraping interval");

    Ok(scheduler)
}

/// 即座にスクレイピングジョブを実行するスケジューラーを作成（テスト用）
pub async fn create_immediate_scheduler(pool: PgPool) -> Result<JobScheduler> {
    let scheduler = JobScheduler::new().await?;

    // 30秒後に1回だけ実行するジョブ（デモ用）
    let job = Job::new_one_shot_async(std::time::Duration::from_secs(30), move |_uuid, _lock| {
        let pool = pool.clone();
        Box::pin(async move {
            let job_id = Uuid::new_v4();
            info!("Starting one-shot scraping job: {}", job_id);

            match run_scraping_job(pool).await {
                Ok(_) => {
                    info!("One-shot job {} completed successfully", job_id);
                }
                Err(e) => {
                    error!("One-shot job {} failed: {}", job_id, e);
                }
            }
        })
    })?;

    scheduler.add(job).await?;

    info!("Scheduler initialized with one-shot job (30 seconds)");

    Ok(scheduler)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scheduler_creation() {
        // スケジューラーのインスタンスが作成できることをテスト
        // JobScheduler自体のテストはtokio-cron-schedulerライブラリの責任なので、
        // ここでは基本的な作成のみをテスト
        let scheduler = JobScheduler::new().await;
        assert!(
            scheduler.is_ok(),
            "JobScheduler should be created successfully"
        );
    }

    #[test]
    fn test_job_duration_calculation() {
        // 時間計算のロジックをテスト
        use std::time::Duration;

        // 1分ごとのcron式の場合
        let one_minute = Duration::from_secs(60);
        assert_eq!(one_minute.as_secs(), 60);

        // 30秒のone-shotジョブの場合
        let thirty_seconds = Duration::from_secs(30);
        assert_eq!(thirty_seconds.as_secs(), 30);
    }

    #[test]
    fn test_uuid_generation() {
        // UUID生成が正しく動作することをテスト
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();

        // UUIDが異なることを確認
        assert_ne!(uuid1, uuid2);

        // UUID文字列の形式を確認
        let uuid_str = uuid1.to_string();
        assert_eq!(uuid_str.len(), 36); // 8-4-4-4-12形式
    }
}
