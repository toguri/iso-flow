use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub id: String, // RSS GUIDまたはリンクのハッシュ
    pub title: String,
    pub description: Option<String>, // RSSの説明文
    pub link: String,
    pub source: NewsSource,
    pub category: String, // "Trade", "Signing", "Other"
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
            NewsSource::Other(s) => write!(f, "{s}"),
        }
    }
}

impl NewsSource {
    pub fn from_string(s: &str) -> Self {
        match s {
            "ESPN" => NewsSource::ESPN,
            "RealGM" => NewsSource::RealGM,
            "HoopsHype" => NewsSource::HoopsHype,
            other => NewsSource::Other(other.to_string()),
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

impl NewsItem {
    /// ニュースのカテゴリーを判定する
    pub fn determine_category(title: &str, description: Option<&str>) -> String {
        let text = format!(
            "{} {}",
            title.to_lowercase(),
            description.unwrap_or("").to_lowercase()
        );

        // トレード関連のキーワード
        let trade_keywords = [
            "trade", "traded", "trading", "acquire", "acquired", "deal", "swap", "move", "moving",
            "sent to", "ships", "sending",
        ];

        // 契約関連のキーワード
        let signing_keywords = [
            "sign",
            "signed",
            "signing",
            "agree",
            "agreed",
            "contract",
            "extension",
            "buyout",
            "waive",
            "waived",
            "release",
            "released",
            "re-sign",
            "resign",
            "pick up option",
            "decline option",
        ];

        // キーワードチェック
        for keyword in &trade_keywords {
            if text.contains(keyword) {
                return "Trade".to_string();
            }
        }

        for keyword in &signing_keywords {
            if text.contains(keyword) {
                return "Signing".to_string();
            }
        }

        "Other".to_string()
    }

    /// IDを生成する（GUIDがない場合はリンクのハッシュを使用）
    pub fn generate_id(guid: Option<&str>, link: &str) -> String {
        if let Some(guid) = guid {
            guid.to_string()
        } else {
            // リンクのハッシュを生成
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher = DefaultHasher::new();
            link.hash(&mut hasher);
            format!("link-{:x}", hasher.finish())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_category_trade() {
        // Trade関連
        assert_eq!(
            NewsItem::determine_category("Lakers trade Russell Westbrook to Jazz", None),
            "Trade"
        );
        assert_eq!(
            NewsItem::determine_category(
                "Nets acquire Ben Simmons",
                Some("The Brooklyn Nets have acquired...")
            ),
            "Trade"
        );
        assert_eq!(
            NewsItem::determine_category("Three-team deal sends Butler to Heat", None),
            "Trade"
        );
    }

    #[test]
    fn test_determine_category_signing() {
        // Signing関連
        assert_eq!(
            NewsItem::determine_category("Kawhi Leonard signs extension with Clippers", None),
            "Signing"
        );
        assert_eq!(
            NewsItem::determine_category("Lakers agree to terms with Austin Reaves", None),
            "Signing"
        );
        assert_eq!(
            NewsItem::determine_category("Wizards waive Isaiah Thomas", None),
            "Signing"
        );
        assert_eq!(
            NewsItem::determine_category("Suns complete buyout with Chris Paul", None),
            "Signing"
        );
    }

    #[test]
    fn test_determine_category_other() {
        // その他
        assert_eq!(
            NewsItem::determine_category("LeBron James scores 40 points in win", None),
            "Other"
        );
        assert_eq!(
            NewsItem::determine_category("NBA announces All-Star starters", None),
            "Other"
        );
    }

    #[test]
    fn test_generate_id() {
        // GUIDがある場合
        assert_eq!(
            NewsItem::generate_id(Some("abc123"), "https://example.com"),
            "abc123"
        );

        // GUIDがない場合はリンクのハッシュ
        let id = NewsItem::generate_id(None, "https://example.com/news/123");
        assert!(id.starts_with("link-"));
        assert!(id.len() > 5);
    }
}
