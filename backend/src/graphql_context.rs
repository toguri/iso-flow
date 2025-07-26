//! GraphQLコンテキストの定義
//!
//! リポジトリトレイトを使ってテスト可能な設計にします。

use crate::db::repository::NewsRepository;
use std::sync::Arc;

/// GraphQLコンテキスト
pub struct GraphQLContext {
    pub news_repository: Arc<dyn NewsRepository>,
}

impl GraphQLContext {
    pub fn new(news_repository: Arc<dyn NewsRepository>) -> Self {
        Self { news_repository }
    }
}
