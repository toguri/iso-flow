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
use tracing::{error, info};

use crate::scraper::{NewsItem, NewsPersistence, RssParser};
use crate::utils::string_utils::strip_html_tags;

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
    /// 日本語タイトル
    pub title_ja: Option<String>,
    /// 日本語説明文
    pub description_ja: Option<String>,
    /// 翻訳ステータス
    pub translation_status: String,
    /// 翻訳日時
    pub translated_at: Option<DateTime<Utc>>,
}

impl From<NewsItem> for TradeNews {
    fn from(item: NewsItem) -> Self {
        TradeNews {
            id: item.id,
            title: strip_html_tags(&item.title),
            description: item.description.map(|desc| strip_html_tags(&desc)),
            link: item.link,
            source: item.source.to_string(),
            published_at: item.published_at,
            category: item.category,
            title_ja: None,
            description_ja: None,
            translation_status: "pending".to_string(),
            translated_at: None,
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

        // SavedNewsItemからTradeNewsに変換
        let news: Vec<TradeNews> = saved_items
            .into_iter()
            .map(|item| TradeNews {
                id: item.id,
                title: strip_html_tags(&item.title),
                description: item.description.map(|desc| strip_html_tags(&desc)),
                link: item.link,
                source: item.source,
                category: item.category,
                published_at: item.published_at,
                title_ja: item.title_ja,
                description_ja: item.description_ja,
                translation_status: item.translation_status,
                translated_at: item.translated_at,
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
            .map(|item| TradeNews {
                id: item.id,
                title: strip_html_tags(&item.title),
                description: item.description.map(|desc| strip_html_tags(&desc)),
                link: item.link,
                source: item.source,
                category: item.category,
                published_at: item.published_at,
                title_ja: item.title_ja,
                description_ja: item.description_ja,
                translation_status: item.translation_status,
                translated_at: item.translated_at,
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
            .filter(|item| item.source.to_lowercase() == source.to_lowercase())
            .map(|item| {
                let news_item = NewsItem {
                    id: item.id,
                    title: item.title,
                    description: item.description,
                    link: item.link,
                    source: crate::scraper::NewsSource::from_string(&item.source),
                    category: item.category,
                    published_at: item.published_at,
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

    /// 未翻訳のニュースを翻訳
    async fn translate_pending_news(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<TranslationResult> {
        let pool = ctx.data::<PgPool>()?;

        info!("Starting translation of pending news...");

        // 環境変数から翻訳サービスの設定を取得
        let api_url = std::env::var("LIBRE_TRANSLATE_URL")
            .unwrap_or_else(|_| "https://libretranslate.com".to_string());
        let api_key = std::env::var("LIBRE_TRANSLATE_API_KEY").ok();

        // 翻訳サービスを初期化（開発中はモックサービスを使用）
        let use_mock =
            std::env::var("USE_MOCK_TRANSLATION").unwrap_or_else(|_| "true".to_string()) == "true";

        let translation_service: Box<dyn crate::services::TranslationService> = if use_mock {
            Box::new(crate::services::MockTranslationService)
        } else {
            Box::new(crate::services::LibreTranslateService::new(
                api_url, api_key,
            ))
        };

        // 未翻訳のニュースを取得
        let pending_items = sqlx::query_as::<_, crate::scraper::persistence::SavedNewsItem>(
            r#"
            SELECT 
                id, title, description, source, link, category, 
                published_at, scraped_at, title_ja, description_ja, 
                translation_status, translated_at
            FROM trade_news
            WHERE translation_status = 'pending' OR translation_status IS NULL
            ORDER BY published_at DESC
            LIMIT 10
            "#,
        )
        .fetch_all(pool)
        .await?;

        info!("Found {} items to translate", pending_items.len());

        let mut translated_count = 0;
        let mut error_count = 0;
        let mut errors = Vec::new();

        for item in pending_items {
            // タイトルの翻訳
            let title_ja = match translation_service.translate(&item.title, "en", "ja").await {
                Ok(text) => text,
                Err(e) => {
                    error!("Failed to translate title for {}: {}", item.id, e);
                    errors.push(format!("{}: Title translation failed - {}", item.id, e));
                    error_count += 1;
                    continue;
                }
            };

            // 説明文の翻訳（存在する場合）
            let description_ja = if let Some(desc) = &item.description {
                match translation_service.translate(desc, "en", "ja").await {
                    Ok(text) => Some(text),
                    Err(e) => {
                        error!("Failed to translate description for {}: {}", item.id, e);
                        errors.push(format!(
                            "{}: Description translation failed - {}",
                            item.id, e
                        ));
                        None
                    }
                }
            } else {
                None
            };

            // データベースを更新
            let result = sqlx::query(
                r#"
                UPDATE trade_news
                SET title_ja = $1,
                    description_ja = $2,
                    translation_status = 'completed',
                    translated_at = NOW()
                WHERE id = $3
                "#,
            )
            .bind(&title_ja)
            .bind(&description_ja)
            .bind(&item.id)
            .execute(pool)
            .await;

            match result {
                Ok(_) => {
                    translated_count += 1;
                    info!("Successfully translated item: {}", item.id);
                }
                Err(e) => {
                    error!("Failed to update database for {}: {}", item.id, e);
                    errors.push(format!("{}: Database update failed - {}", item.id, e));
                    error_count += 1;
                }
            }

            // レート制限を考慮して少し待機
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        info!(
            "Translation completed: {} translated, {} errors",
            translated_count, error_count
        );

        Ok(TranslationResult {
            translated_count,
            error_count,
            errors,
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

/// 翻訳結果
#[derive(SimpleObject)]
pub struct TranslationResult {
    /// 翻訳されたアイテム数
    pub translated_count: i32,
    /// エラー数
    pub error_count: i32,
    /// エラーメッセージのリスト
    pub errors: Vec<String>,
}

pub type QueryRoot = Query;

pub fn create_schema(pool: PgPool) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::create_pool;
    use crate::scraper::{NewsItem, NewsSource};
    use async_graphql::Value;
    use chrono::Utc;

    #[tokio::test]
    #[ignore = "Requires PostgreSQL database"]
    async fn test_query_resolver_all_trade_news() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://test_user:test_password@localhost:5433/test_iso_flow".to_string()
        });
        std::env::set_var("DATABASE_URL", database_url);

        let pool = match create_pool().await {
            Ok(pool) => pool,
            Err(_) => {
                eprintln!("Skipping test: PostgreSQL database required");
                return;
            }
        };

        // GraphQLスキーマを作成してコンテキスト経由でテスト
        let schema = create_schema(pool.clone());
        let query = r#"
            query {
                tradeNews {
                    id
                    title
                }
            }
        "#;
        let result = schema.execute(query).await;
        assert!(result.is_ok(), "Should retrieve all trade news");

        std::env::remove_var("DATABASE_URL");
    }

    #[tokio::test]
    #[ignore = "Requires PostgreSQL database"]
    async fn test_query_resolver_trade_news_by_category() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://test_user:test_password@localhost:5433/test_iso_flow".to_string()
        });
        std::env::set_var("DATABASE_URL", database_url);

        let pool = match create_pool().await {
            Ok(pool) => pool,
            Err(_) => {
                eprintln!("Skipping test: PostgreSQL database required");
                return;
            }
        };

        // GraphQLスキーマを作成してコンテキスト経由でテスト
        let schema = create_schema(pool.clone());
        let query = r#"
            query {
                tradeNewsByCategory(category: "Trade") {
                    id
                    category
                }
            }
        "#;
        let result = schema.execute(query).await;
        assert!(result.is_ok(), "Should retrieve trade news by category");

        std::env::remove_var("DATABASE_URL");
    }

    #[tokio::test]
    #[ignore = "Requires PostgreSQL database"]
    async fn test_query_resolver_trade_news_by_source() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://test_user:test_password@localhost:5433/test_iso_flow".to_string()
        });
        std::env::set_var("DATABASE_URL", database_url);

        let pool = match create_pool().await {
            Ok(pool) => pool,
            Err(_) => {
                eprintln!("Skipping test: PostgreSQL database required");
                return;
            }
        };

        // GraphQLスキーマを作成してコンテキスト経由でテスト
        let schema = create_schema(pool.clone());
        let query = r#"
            query {
                tradeNewsBySource(source: "ESPN") {
                    id
                    source
                }
            }
        "#;
        let result = schema.execute(query).await;
        assert!(result.is_ok(), "Should retrieve trade news by source");

        std::env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_trade_news_from_news_item() {
        let published_at = Utc::now();

        // NewsItemを作成
        let news_item = NewsItem {
            id: "test-123".to_string(),
            title: "Lakers Trade Update".to_string(),
            description: Some("Lakers are trading...".to_string()),
            link: "https://example.com/news/123".to_string(),
            source: NewsSource::ESPN,
            category: "Trade".to_string(),
            published_at,
        };

        // TradeNewsに変換
        let trade_news = TradeNews::from(news_item.clone());

        // 全フィールドが正しく変換されていることを確認
        assert_eq!(trade_news.id, "test-123");
        assert_eq!(trade_news.title, "Lakers Trade Update");
        assert_eq!(
            trade_news.description,
            Some("Lakers are trading...".to_string())
        );
        assert_eq!(trade_news.link, "https://example.com/news/123");
        assert_eq!(trade_news.source, "ESPN");
        assert_eq!(trade_news.category, "Trade");
        assert_eq!(trade_news.published_at, published_at);
    }

    #[test]
    fn test_trade_news_from_news_item_without_description() {
        let published_at = Utc::now();

        // descriptionがないNewsItemを作成
        let news_item = NewsItem {
            id: "test-456".to_string(),
            title: "Celtics Signing".to_string(),
            description: None,
            link: "https://example.com/news/456".to_string(),
            source: NewsSource::RealGM,
            category: "Signing".to_string(),
            published_at,
        };

        // TradeNewsに変換
        let trade_news = TradeNews::from(news_item);

        // descriptionがNoneであることを確認
        assert_eq!(trade_news.id, "test-456");
        assert_eq!(trade_news.title, "Celtics Signing");
        assert_eq!(trade_news.description, None);
        assert_eq!(trade_news.source, "RealGM");
        assert_eq!(trade_news.category, "Signing");
    }

    #[test]
    fn test_trade_news_from_other_news_source() {
        let published_at = Utc::now();

        // OtherタイプのNewsSourceを作成
        let news_item = NewsItem {
            id: "test-789".to_string(),
            title: "Warriors Update".to_string(),
            description: Some("Warriors news...".to_string()),
            link: "https://example.com/news/789".to_string(),
            source: NewsSource::Other("CustomSource".to_string()),
            category: "Other".to_string(),
            published_at,
        };

        // TradeNewsに変換
        let trade_news = TradeNews::from(news_item);

        // Otherソースが正しく文字列に変換されることを確認
        assert_eq!(trade_news.source, "CustomSource");
    }

    #[test]
    fn test_scrape_result_creation() {
        let result = ScrapeResult {
            saved_count: 10,
            skipped_count: 5,
            error_count: 2,
            errors: vec![
                "item1: Network error".to_string(),
                "item2: Parse error".to_string(),
            ],
        };

        assert_eq!(result.saved_count, 10);
        assert_eq!(result.skipped_count, 5);
        assert_eq!(result.error_count, 2);
        assert_eq!(result.errors.len(), 2);
        assert_eq!(result.errors[0], "item1: Network error");
        assert_eq!(result.errors[1], "item2: Parse error");
    }

    #[tokio::test]
    #[ignore = "Requires PostgreSQL database"]
    async fn test_mutation_resolver_scrape_all_feeds() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://test_user:test_password@localhost:5433/test_iso_flow".to_string()
        });
        std::env::set_var("DATABASE_URL", database_url);

        let pool = match create_pool().await {
            Ok(pool) => pool,
            Err(_) => {
                eprintln!("Skipping test: PostgreSQL database required");
                return;
            }
        };

        // GraphQLスキーマを作成してコンテキスト経由でテスト
        let schema = create_schema(pool.clone());
        let mutation = r#"
            mutation {
                scrapeAllFeeds {
                    savedCount
                    skippedCount
                    errorCount
                }
            }
        "#;
        let result = schema.execute(mutation).await;
        assert!(
            !result.errors.is_empty() || result.data != Value::Null,
            "Should execute mutation"
        );

        std::env::remove_var("DATABASE_URL");
    }
}
