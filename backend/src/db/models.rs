use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub name_ja: Option<String>,
    pub conference: String,
    pub division: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub name_ja: Option<String>,
    pub position: Option<String>,
    pub current_team_id: Option<i32>,
    pub jersey_number: Option<i32>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradeNewsDb {
    pub id: i32,
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub title_ja: Option<String>,
    pub description_ja: Option<String>,
    pub translation_status: String,
    pub translated_at: Option<DateTime<Utc>>,
    pub source_name: String,
    pub source_url: String,
    pub author: Option<String>,
    pub category: String,
    pub is_official: bool,
    pub published_at: DateTime<Utc>,
    pub scraped_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RawFeedData {
    pub id: i32,
    pub source_name: String,
    pub feed_url: String,
    pub raw_content: String,
    pub fetched_at: DateTime<Utc>,
}

// Insert用の構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTradeNews {
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub source_name: String,
    pub source_url: String,
    pub author: Option<String>,
    pub category: String,
    pub is_official: bool,
    pub published_at: DateTime<Utc>,
}
