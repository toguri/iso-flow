use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// チーム情報
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub name_ja: Option<String>,
    pub conference: String,
    pub division: String,
    pub created_at: String,
    pub updated_at: String,
}

/// トレードニュース
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradeNews {
    pub id: i64,
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub title_ja: Option<String>,
    pub description_ja: Option<String>,
    pub translation_status: String,
    pub translated_at: Option<String>,
    pub source_name: String,
    pub source_url: String,
    pub author: Option<String>,
    pub category: String,
    pub is_official: Option<bool>,
    pub published_at: String,
    pub scraped_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// トレードニュース作成用の入力データ
#[derive(Debug, Clone)]
pub struct NewTradeNews {
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub source_name: String,
    pub source_url: String,
    pub category: String,
    pub published_at: DateTime<Utc>,
}
