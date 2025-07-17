use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeNews {
    pub id: String,
    pub title: String,
    pub description: String,
    pub link: String,
    pub published_at: DateTime<Utc>,
    pub source: NewsSource,
    pub is_trade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NewsSource {
    ESPN,
    RealGM,
    HoopsHype,
    Other(String),
}

impl std::fmt::Display for NewsSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewsSource::ESPN => write!(f, "ESPN"),
            NewsSource::RealGM => write!(f, "RealGM"),
            NewsSource::HoopsHype => write!(f, "HoopsHype"),
            NewsSource::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RssFeed {
    pub url: String,
    pub source: NewsSource,
}

impl RssFeed {
    pub fn new(url: &str, source: NewsSource) -> Self {
        Self {
            url: url.to_string(),
            source,
        }
    }
}

pub const RSS_FEEDS: &[(&str, NewsSource)] = &[
    ("https://www.espn.com/espn/rss/nba/news", NewsSource::ESPN),
    (
        "https://basketball.realgm.com/rss/wiretap/0/0.xml",
        NewsSource::RealGM,
    ),
];
