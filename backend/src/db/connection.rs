use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, AnyPool};
use tracing::info;

/// データベース接続プールを作成
/// DATABASE_URLの形式に基づいて、SQLiteまたはPostgreSQLに接続
pub async fn create_pool() -> Result<AnyPool> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:nba_trades.db".to_string());

    info!(
        "Connecting to database: {}",
        mask_connection_string(&database_url)
    );

    // データベースが存在しない場合は作成（SQLiteの場合）
    if database_url.starts_with("sqlite:") {
        if !sqlx::Sqlite::database_exists(&database_url).await? {
            info!("Creating SQLite database...");
            sqlx::Sqlite::create_database(&database_url).await?;
        }
    }

    // 接続プールを作成
    let pool = AnyPool::connect(&database_url).await?;

    // データベースの種類を確認
    if database_url.starts_with("sqlite:") {
        info!("Connected to SQLite database");
    } else if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        info!("Connected to PostgreSQL database");
    } else {
        info!("Connected to database");
    }

    Ok(pool)
}

/// データベース種別を取得
#[allow(dead_code)]
pub fn get_database_type(_pool: &AnyPool) -> &'static str {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:nba_trades.db".to_string());

    if database_url.starts_with("sqlite:") {
        "sqlite"
    } else if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        "postgres"
    } else {
        "unknown"
    }
}

/// 接続文字列をマスク（セキュリティのため）
fn mask_connection_string(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(scheme_end) = url.find("://") {
            let scheme = &url[..scheme_end + 3];
            let host_part = &url[at_pos..];
            return format!("{scheme}****{host_part}");
        }
    }

    // SQLiteの場合はそのまま返す
    if url.starts_with("sqlite:") {
        return url.to_string();
    }

    // その他の場合は一部をマスク
    let masked_part = &url[..url.len().min(20)];
    format!("{masked_part}...masked")
}
