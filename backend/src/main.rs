use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{response::Html, routing::get, serve, Router};
use nba_trade_scraper::graphql::{create_schema, Query};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

async fn graphql_handler(
    schema: axum::extract::Extension<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ログの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting GraphQL server...");

    // データベース接続の初期化
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:nba_trades.db".to_string());
    let pool = SqlitePool::connect(&database_url).await?;

    // マイグレーション実行
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database initialized");

    let schema = create_schema(pool);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(axum::extract::Extension(schema));

    info!("GraphQL playground available at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;

    Ok(())
}
