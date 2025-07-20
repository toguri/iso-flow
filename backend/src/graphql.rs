use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use chrono::{DateTime, Utc};

use crate::scraper::{RssParser, TradeNews as ScraperTradeNews};

#[derive(SimpleObject)]
pub struct TradeNews {
    pub id: String,
    pub title: String,
    pub link: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
    pub category: String,
}

impl From<ScraperTradeNews> for TradeNews {
    fn from(item: ScraperTradeNews) -> Self {
        let category = if item.title.to_lowercase().contains("trade")
            || item.title.to_lowercase().contains("acquire")
            || item.title.to_lowercase().contains("deal")
        {
            "Trade".to_string()
        } else if item.title.to_lowercase().contains("sign")
            || item.title.to_lowercase().contains("agree")
            || item.title.to_lowercase().contains("buyout")
        {
            "Signing".to_string()
        } else {
            "Other".to_string()
        };

        TradeNews {
            id: item.id,
            title: item.title,
            link: item.link,
            source: item.source.to_string(),
            published_at: item.published_at,
            category,
        }
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn trade_news(&self, _ctx: &Context<'_>) -> async_graphql::Result<Vec<TradeNews>> {
        let parser = RssParser::new();
        let news = parser.fetch_all_feeds().await?;

        Ok(news.into_iter().map(TradeNews::from).collect())
    }

    async fn trade_news_by_category(
        &self,
        _ctx: &Context<'_>,
        category: String,
    ) -> async_graphql::Result<Vec<TradeNews>> {
        let parser = RssParser::new();
        let news = parser.fetch_all_feeds().await?;

        let trade_news: Vec<TradeNews> = news.into_iter().map(TradeNews::from).collect();

        Ok(trade_news
            .into_iter()
            .filter(|n| n.category.to_lowercase() == category.to_lowercase())
            .collect())
    }

    async fn trade_news_by_source(
        &self,
        _ctx: &Context<'_>,
        source: String,
    ) -> async_graphql::Result<Vec<TradeNews>> {
        let parser = RssParser::new();
        let news = parser.fetch_all_feeds().await?;

        let trade_news: Vec<TradeNews> = news.into_iter().map(TradeNews::from).collect();

        Ok(trade_news
            .into_iter()
            .filter(|n| n.source.to_lowercase() == source.to_lowercase())
            .collect())
    }
}

pub type QueryRoot = Query;

pub fn create_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::new(Query, EmptyMutation, EmptySubscription)
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
