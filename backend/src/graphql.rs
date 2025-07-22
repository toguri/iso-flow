//! GraphQL APIの実装
//!
//! このモジュールは、NBAトレード情報を提供するGraphQL APIを実装します。
//!
//! ## クエリ
//!
//! - `tradeNews`: 全てのトレードニュースを取得
//! - `tradeNewsByCategory`: カテゴリー別にニュースを取得
//! - `tradeNewsBySource`: ソース別にニュースを取得

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::scraper::{NewsItem, NewsPersistence};

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
        let pool = ctx.data::<SqlitePool>()?;
        let persistence = NewsPersistence::new(pool.clone());
        
        let saved_items = persistence.get_recent_news(100).await?;
        
        // SavedNewsItemからNewsItemに変換してからTradeNewsに変換
        let news: Vec<TradeNews> = saved_items
            .into_iter()
            .map(|item| {
                // time::OffsetDateTimeからchrono::DateTime<Utc>に変換
                let unix_timestamp = item.published_at.unix_timestamp();
                let published_at = DateTime::<Utc>::from_timestamp(unix_timestamp, 0)
                    .unwrap_or_else(Utc::now);
                
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
        let pool = ctx.data::<SqlitePool>()?;
        let persistence = NewsPersistence::new(pool.clone());
        
        let saved_items = persistence.get_news_by_category(&category).await?;
        
        let news: Vec<TradeNews> = saved_items
            .into_iter()
            .map(|item| {
                // time::OffsetDateTimeからchrono::DateTime<Utc>に変換
                let unix_timestamp = item.published_at.unix_timestamp();
                let published_at = DateTime::<Utc>::from_timestamp(unix_timestamp, 0)
                    .unwrap_or_else(Utc::now);
                
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
        let pool = ctx.data::<SqlitePool>()?;
        let persistence = NewsPersistence::new(pool.clone());
        
        // ソース別のフィルタリングは現在のpersistenceに実装されていないので、
        // 全件取得してフィルタリング
        let saved_items = persistence.get_recent_news(200).await?;
        
        let news: Vec<TradeNews> = saved_items
            .into_iter()
            .filter(|item| item.source_name.to_lowercase() == source.to_lowercase())
            .map(|item| {
                // time::OffsetDateTimeからchrono::DateTime<Utc>に変換
                let unix_timestamp = item.published_at.unix_timestamp();
                let published_at = DateTime::<Utc>::from_timestamp(unix_timestamp, 0)
                    .unwrap_or_else(Utc::now);
                
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

pub type QueryRoot = Query;

pub fn create_schema(pool: SqlitePool) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}

pub fn graphql_routes(schema: Schema<Query, EmptyMutation, EmptySubscription>) -> axum::Router {
    use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
    use axum::{extract::State, response::Html, routing::get, Router};

    async fn graphql_handler(
        State(schema): State<Schema<Query, EmptyMutation, EmptySubscription>>,
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
