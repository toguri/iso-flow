use anyhow::Result;
use sqlx::{Any, AnyPool, migrate::MigrateDatabase};
use tracing::{info, warn};

/// データベース接続プールを作成
/// DATABASE_URLの形式に基づいて、SQLiteまたはPostgreSQLに接続
pub async fn create_pool() -> Result<AnyPool> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:nba_trades.db".to_string());
    
    info!("Connecting to database: {}", mask_connection_string(&database_url));
    
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
    match pool.any_kind() {
        sqlx::any::AnyKind::Sqlite => {
            info!("Connected to SQLite database");
        }
        sqlx::any::AnyKind::Postgres => {
            info!("Connected to PostgreSQL database");
        }
        _ => {
            warn!("Connected to unsupported database type");
        }
    }
    
    Ok(pool)
}

/// データベース種別を取得
pub fn get_database_type(pool: &AnyPool) -> &'static str {
    match pool.any_kind() {
        sqlx::any::AnyKind::Sqlite => "sqlite",
        sqlx::any::AnyKind::Postgres => "postgres",
        _ => "unknown",
    }
}

/// 接続文字列をマスク（セキュリティのため）
fn mask_connection_string(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(scheme_end) = url.find("://") {
            let scheme = &url[..scheme_end + 3];
            let host_part = &url[at_pos..];
            return format!("{}****{}", scheme, host_part);
        }
    }
    
    // SQLiteの場合はそのまま返す
    if url.starts_with("sqlite:") {
        return url.to_string();
    }
    
    // その他の場合は一部をマスク
    format!("{}...masked", &url[..url.len().min(20)])
}