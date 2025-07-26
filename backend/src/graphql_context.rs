//! GraphQLコンテキストの定義
//!
//! リポジトリトレイトを使ってテスト可能な設計にします。

use std::sync::Arc;
use crate::db::repository::NewsRepository;

/// GraphQLコンテキスト
pub struct GraphQLContext {
    pub news_repository: Arc<dyn NewsRepository>,
}

impl GraphQLContext {
    pub fn new(news_repository: Arc<dyn NewsRepository>) -> Self {
        Self { news_repository }
    }
}