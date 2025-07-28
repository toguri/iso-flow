//! 永続化層のトレイト定義
//!
//! テスト可能な設計のためにトレイトを定義します。

use anyhow::Result;
use async_trait::async_trait;
use crate::scraper::models::NewsItem;

/// ニュース永続化のトレイト
#[async_trait]
pub trait NewsPersistenceTrait: Send + Sync {
    /// ニュースアイテムを保存
    async fn save_news_items(&self, items: Vec<NewsItem>) -> Result<SaveResult>;
    
    /// 外部IDで存在確認
    async fn exists_by_external_id(&self, external_id: &str) -> Result<bool>;
    
    /// 最新のニュースを取得
    async fn get_recent_news(&self, limit: i32) -> Result<Vec<SavedNewsItem>>;
    
    /// カテゴリー別にニュースを取得
    async fn get_news_by_category(&self, category: &str) -> Result<Vec<SavedNewsItem>>;
}

/// 保存結果
#[derive(Debug, Clone)]
pub struct SaveResult {
    pub saved_count: usize,
    pub skipped_count: usize,
    pub errors: Vec<(String, String)>,
}

/// データベースに保存されたニュースアイテム
#[derive(Debug, Clone)]
pub struct SavedNewsItem {
    pub id: Option<i64>,
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub source_name: String,
    pub source_url: String,
    pub category: String,
    pub is_official: Option<bool>,
    pub published_at: String,
    pub scraped_at: Option<String>,
    pub created_at: Option<String>,
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    /// モック永続化実装
    pub struct MockNewsPersistence {
        items: Arc<Mutex<HashMap<String, SavedNewsItem>>>,
        should_fail: Arc<Mutex<bool>>,
        fail_on_ids: Arc<Mutex<Vec<String>>>,
    }

    impl MockNewsPersistence {
        pub fn new() -> Self {
            Self {
                items: Arc::new(Mutex::new(HashMap::new())),
                should_fail: Arc::new(Mutex::new(false)),
                fail_on_ids: Arc::new(Mutex::new(Vec::new())),
            }
        }

        /// 特定のIDで失敗するように設定
        pub fn fail_on_id(&self, id: String) {
            self.fail_on_ids.lock().unwrap().push(id);
        }

        /// すべての操作を失敗させる
        pub fn set_should_fail(&self, should_fail: bool) {
            *self.should_fail.lock().unwrap() = should_fail;
        }

        /// 既存のアイテムを追加
        pub fn add_existing_item(&self, id: String, item: SavedNewsItem) {
            self.items.lock().unwrap().insert(id, item);
        }
    }

    #[async_trait]
    impl NewsPersistenceTrait for MockNewsPersistence {
        async fn save_news_items(&self, items: Vec<NewsItem>) -> Result<SaveResult> {
            if *self.should_fail.lock().unwrap() {
                return Err(anyhow::anyhow!("Mock failure: Database connection error"));
            }

            let mut saved_count = 0;
            let mut skipped_count = 0;
            let mut errors = Vec::new();
            
            // fail_on_idsをクローンしてMutexガードを早期に解放
            let fail_on_ids: Vec<String> = self.fail_on_ids.lock().unwrap().clone();

            for item in items {
                // 特定のIDで失敗
                if fail_on_ids.contains(&item.id) {
                    errors.push((item.id.clone(), "Mock error: Failed to save specific item".to_string()));
                    continue;
                }

                // 既存チェック
                if self.exists_by_external_id(&item.id).await? {
                    skipped_count += 1;
                } else {
                    // 保存
                    let saved_item = SavedNewsItem {
                        id: Some(1),
                        external_id: item.id.clone(),
                        title: item.title,
                        description: item.description,
                        source_name: item.source.to_string(),
                        source_url: item.link,
                        category: item.category,
                        is_official: Some(false),
                        published_at: item.published_at.to_rfc3339(),
                        scraped_at: Some(chrono::Utc::now().to_rfc3339()),
                        created_at: Some(chrono::Utc::now().to_rfc3339()),
                    };
                    
                    self.items.lock().unwrap().insert(item.id, saved_item);
                    saved_count += 1;
                }
            }

            Ok(SaveResult {
                saved_count,
                skipped_count,
                errors,
            })
        }

        async fn exists_by_external_id(&self, external_id: &str) -> Result<bool> {
            if *self.should_fail.lock().unwrap() {
                return Err(anyhow::anyhow!("Mock failure: Database query error"));
            }
            
            Ok(self.items.lock().unwrap().contains_key(external_id))
        }

        async fn get_recent_news(&self, limit: i32) -> Result<Vec<SavedNewsItem>> {
            if *self.should_fail.lock().unwrap() {
                return Err(anyhow::anyhow!("Mock failure: Database query error"));
            }

            let items = self.items.lock().unwrap();
            let mut result: Vec<SavedNewsItem> = items.values().cloned().collect();
            result.sort_by(|a, b| b.published_at.cmp(&a.published_at));
            result.truncate(limit as usize);
            Ok(result)
        }

        async fn get_news_by_category(&self, category: &str) -> Result<Vec<SavedNewsItem>> {
            if *self.should_fail.lock().unwrap() {
                return Err(anyhow::anyhow!("Mock failure: Database query error"));
            }

            let items = self.items.lock().unwrap();
            let result: Vec<SavedNewsItem> = items
                .values()
                .filter(|item| item.category == category)
                .cloned()
                .collect();
            Ok(result)
        }
    }
}