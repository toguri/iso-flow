use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{response::Html, routing::get, serve, Router};
use nba_trade_scraper::graphql::{create_schema, Query, Mutation};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

async fn graphql_handler(
    schema: axum::extract::Extension<Schema<Query, Mutation, EmptySubscription>>,
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

    // スケジューラーを起動（5分間隔）
    let scheduler_pool = pool.clone();
    let scheduler_handle = tokio::spawn(async move {
        match nba_trade_scraper::scheduler::create_scheduler(scheduler_pool).await {
            Ok(scheduler) => {
                info!("Scheduler created successfully");
                if let Err(e) = scheduler.start().await {
                    error!("Failed to start scheduler: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to create scheduler: {}", e);
            }
        }
    });

    info!("Scheduler starting with 5 minute interval");

    let schema = create_schema(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(cors)
        .layer(axum::extract::Extension(schema));

    info!("GraphQL playground available at http://localhost:8000");

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    
    // サーバーとスケジューラーを並行実行
    tokio::select! {
        result = serve(listener, app) => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = scheduler_handle => {
            error!("Scheduler unexpectedly stopped");
        }
    }

    Ok(())
}
