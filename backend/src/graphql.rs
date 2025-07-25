//! GraphQL APIの実装
//!
//! このモジュールは、NBAトレード情報を提供するGraphQL APIを実装します。
//!
//! ## クエリ
//!
//! - `tradeNews`: 全てのトレードニュースを取得
//! - `tradeNewsByCategory`: カテゴリー別にニュースを取得
//! - `tradeNewsBySource`: ソース別にニュースを取得

use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use tracing::info;

use crate::scraper::{NewsItem, NewsPersistence, RssParser};

/// GraphQLで返されるトレードニュースの構造体
#[derive(SimpleObject)]
pub struct TradeNews {
    /// ニュースの一意識別子
    pub id: String,
    /// ニュースのタイトル
    pub title: String,
    /// ニュースの説明文
    pub description: Option<String>,
    /// ニュースへのリンク
    pub link: String,
    /// ニュースソース（ESPN、RealGMなど）
    pub source: String,
    /// 公開日時
    pub published_at: DateTime<Utc>,
    /// カテゴリー（Trade、Signing、Other）
    pub category: String,
}

impl From<NewsItem> for TradeNews {
    fn from(item: NewsItem) -> Self {
        TradeNews {
            id: item.id,
            title: item.title,
            description: item.description,
            link: item.link,
            source: item.source.to_string(),
            published_at: item.published_at,
            category: item.category,
        }
    }
}

/// GraphQLクエリのルート
pub struct Query;

#[Object]
impl Query {
    /// 全てのトレードニュースを取得します（データベースから）
    ///
    /// 最新100件のニュースを返します
    async fn trade_news(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<TradeNews>> {
        let pool = ctx.data::<PgPool>()?;
        let persistence = NewsPersistence::new(pool.clone());

        let saved_items = persistence.get_recent_news(100).await?;

        // SavedNewsItemからNewsItemに変換してからTradeNewsに変換
        let news: Vec<TradeNews> = saved_items
            .into_iter()
            .map(|item| {
                // RFC3339文字列からchrono::DateTime<Utc>に変換
                let published_at = DateTime::parse_from_rfc3339(&item.published_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                let news_item = NewsItem {
                    id: item.external_id,
                    title: item.title,
                    description: item.description,
                    link: item.source_url,
                    source: crate::scraper::NewsSource::from_string(&item.source_name),
                    category: item.category,
                    published_at,
                };
                TradeNews::from(news_item)
            })
            .collect();

        Ok(news)
    }

    async fn trade_news_by_category(
        &self,
        ctx: &Context<'_>,
        category: String,
    ) -> async_graphql::Result<Vec<TradeNews>> {
        let pool = ctx.data::<PgPool>()?;
        let persistence = NewsPersistence::new(pool.clone());

        let saved_items = persistence.get_news_by_category(&category).await?;

        let news: Vec<TradeNews> = saved_items
            .into_iter()
            .map(|item| {
                // RFC3339文字列からchrono::DateTime<Utc>に変換
                let published_at = DateTime::parse_from_rfc3339(&item.published_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                let news_item = NewsItem {
                    id: item.external_id,
                    title: item.title,
                    description: item.description,
                    link: item.source_url,
                    source: crate::scraper::NewsSource::from_string(&item.source_name),
                    category: item.category,
                    published_at,
                };
                TradeNews::from(news_item)
            })
            .collect();

        Ok(news)
    }

    async fn trade_news_by_source(
        &self,
        ctx: &Context<'_>,
        source: String,
    ) -> async_graphql::Result<Vec<TradeNews>> {
        let pool = ctx.data::<PgPool>()?;
        let persistence = NewsPersistence::new(pool.clone());

        // ソース別のフィルタリングは現在のpersistenceに実装されていないので、
        // 全件取得してフィルタリング
        let saved_items = persistence.get_recent_news(200).await?;

        let news: Vec<TradeNews> = saved_items
            .into_iter()
            .filter(|item| item.source_name.to_lowercase() == source.to_lowercase())
            .map(|item| {
                // RFC3339文字列からchrono::DateTime<Utc>に変換
                let published_at = DateTime::parse_from_rfc3339(&item.published_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                let news_item = NewsItem {
                    id: item.external_id,
                    title: item.title,
                    description: item.description,
                    link: item.source_url,
                    source: crate::scraper::NewsSource::from_string(&item.source_name),
                    category: item.category,
                    published_at,
                };
                TradeNews::from(news_item)
            })
            .collect();

        Ok(news)
    }
}

/// GraphQLミューテーションのルート
pub struct Mutation;

#[Object]
impl Mutation {
    /// RSSフィードをスクレイピングしてデータベースに保存
    async fn scrape_rss(&self, ctx: &Context<'_>) -> async_graphql::Result<ScrapeResult> {
        let pool = ctx.data::<PgPool>()?;
        let persistence = NewsPersistence::new(pool.clone());

        info!("Starting RSS scraping via GraphQL mutation...");

        // RSSフィードをパース
        let parser = RssParser::new();
        let news_items = parser.fetch_all_feeds().await?;

        info!("Fetched {} items from RSS feeds", news_items.len());

        // データベースに保存
        let save_result = persistence.save_news_items(news_items).await?;

        info!(
            "Scraping completed: {} saved, {} skipped, {} errors",
            save_result.saved_count,
            save_result.skipped_count,
            save_result.errors.len()
        );

        Ok(ScrapeResult {
            saved_count: save_result.saved_count as i32,
            skipped_count: save_result.skipped_count as i32,
            error_count: save_result.errors.len() as i32,
            errors: save_result
                .errors
                .into_iter()
                .map(|(id, msg)| format!("{id}: {msg}"))
                .collect(),
        })
    }
}

/// スクレイピング結果
#[derive(SimpleObject)]
pub struct ScrapeResult {
    /// 新規保存されたアイテム数
    pub saved_count: i32,
    /// 重複のためスキップされたアイテム数
    pub skipped_count: i32,
    /// エラー数
    pub error_count: i32,
    /// エラーメッセージのリスト
    pub errors: Vec<String>,
}

pub type QueryRoot = Query;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scraper::{NewsItem, NewsSource};
    use chrono::Utc;

    #[test]
    fn test_trade_news_from_news_item() {
        // NewsItemを作成
        let news_item = NewsItem {
            id: "test-123".to_string(),
            title: "Lakers Trade Update".to_string(),
            description: Some("Lakers are trading...".to_string()),
            link: "https://example.com/news/123".to_string(),
            source: NewsSource::ESPN,
            category: "Trade".to_string(),
            published_at: Utc::now(),
        };

        // TradeNewsに変換
        let trade_news = TradeNews::from(news_item.clone());

        // 各フィールドが正しく変換されていることを確認
        assert_eq!(trade_news.id, news_item.id);
        assert_eq!(trade_news.title, news_item.title);
        assert_eq!(trade_news.description, news_item.description);
        assert_eq!(trade_news.link, news_item.link);
        assert_eq!(trade_news.source, "ESPN");
        assert_eq!(trade_news.published_at, news_item.published_at);
        assert_eq!(trade_news.category, news_item.category);
    }

    #[test]
    fn test_trade_news_from_news_item_with_other_source() {
        // Other sourceタイプのNewsItemを作成
        let news_item = NewsItem {
            id: "test-456".to_string(),
            title: "NBA News".to_string(),
            description: None,
            link: "https://example.com/news/456".to_string(),
            source: NewsSource::Other("Custom Source".to_string()),
            category: "Other".to_string(),
            published_at: Utc::now(),
        };

        // TradeNewsに変換
        let trade_news = TradeNews::from(news_item);

        // sourceがCustom Sourceとして正しく表示されることを確認
        assert_eq!(trade_news.source, "Custom Source");
        assert_eq!(trade_news.description, None);
    }

    #[test]
    fn test_scrape_result_creation() {
        let result = ScrapeResult {
            saved_count: 10,
            skipped_count: 5,
            error_count: 2,
            errors: vec![
                "error1: Failed to save".to_string(),
                "error2: Network error".to_string(),
            ],
        };

        assert_eq!(result.saved_count, 10);
        assert_eq!(result.skipped_count, 5);
        assert_eq!(result.error_count, 2);
        assert_eq!(result.errors.len(), 2);
    }
}

pub fn create_schema(pool: PgPool) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish()
}

/// リポジトリを使用してGraphQLスキーマを作成（テスト用）
#[cfg(any(test, feature = "test-utils"))]
pub fn create_schema_with_repository(
    repository: std::sync::Arc<dyn crate::db::repository::NewsRepository>,
) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(repository)
        .finish()
}

pub fn graphql_routes(schema: Schema<Query, Mutation, EmptySubscription>) -> axum::Router {
    use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
    use axum::{extract::State, response::Html, routing::get, Router};

    async fn graphql_handler(
        State(schema): State<Schema<Query, Mutation, EmptySubscription>>,
        req: GraphQLRequest,
    ) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }

    async fn graphql_playground() -> Html<String> {
        Html(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/"),
        ))
    }

    Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .with_state(schema)
}
