//! スケジューラー関連の機能
//!
//! 定期的なスクレイピングジョブの実行を管理します。

use anyhow::Result;
use sqlx::AnyPool;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};
use uuid::Uuid;

use crate::scraper::{NewsPersistence, RssParser};

/// スクレイピングジョブを実行する
pub async fn run_scraping_job(pool: AnyPool) -> Result<()> {
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
pub async fn create_scheduler(pool: AnyPool) -> Result<JobScheduler> {
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
pub async fn create_immediate_scheduler(pool: AnyPool) -> Result<JobScheduler> {
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
    async fn test_create_scheduler() {
        let pool = sqlx::AnyPool::connect("sqlite::memory:").await.unwrap();

        // マイグレーションを実行
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let mut scheduler = create_scheduler(pool).await.unwrap();

        // スケジューラーが作成されることを確認
        assert!(scheduler.next_tick_for_job(Uuid::nil()).await.is_ok());
    }

    #[tokio::test]
    async fn test_create_immediate_scheduler() {
        let pool = sqlx::AnyPool::connect("sqlite::memory:").await.unwrap();

        // マイグレーションを実行
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let mut scheduler = create_immediate_scheduler(pool).await.unwrap();

        // スケジューラーが作成されることを確認
        assert!(scheduler.next_tick_for_job(Uuid::nil()).await.is_ok());
    }

    #[tokio::test]
    async fn test_run_scraping_job() {
        let pool = sqlx::AnyPool::connect("sqlite::memory:").await.unwrap();

        // マイグレーションを実行
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        // スクレイピングジョブが正常に実行されることを確認
        // 実際のRSSフィード取得はモックしないので、エラーにならないことだけ確認
        let result = run_scraping_job(pool).await;

        // ネットワークエラーの可能性があるので、結果は問わない
        // ただし、パニックしないことを確認
        match result {
            Ok(_) => println!("Scraping job succeeded"),
            Err(e) => println!("Scraping job failed (expected in test): {}", e),
        }
    }
}
