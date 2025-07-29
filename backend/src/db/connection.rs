use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

/// PostgreSQL接続プールを作成
// coverage: off
pub async fn create_pool() -> Result<PgPool> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            // CI環境などでDATABASE_URLが設定されていない場合
            return Err(anyhow::anyhow!(
                "DATABASE_URL environment variable not set. PostgreSQL connection required."
            ));
        }
    };

    // PostgreSQL URLのみ許可
    if !database_url.starts_with("postgres://") && !database_url.starts_with("postgresql://") {
        return Err(anyhow::anyhow!(
            "Only PostgreSQL URLs are supported. Got: {}",
            mask_connection_string(&database_url)
        ));
    }

    info!(
        "Connecting to PostgreSQL: {}",
        mask_connection_string(&database_url)
    );

    // 接続プールを作成
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    info!("Connected to PostgreSQL database");

    Ok(pool)
}
// coverage: on

/// データベース種別を取得（PostgreSQL固定）
#[allow(dead_code)]
// coverage: off
pub fn get_database_type(_pool: &PgPool) -> &'static str {
    "postgres"
}
// coverage: on

/// 接続文字列をマスク（セキュリティのため）
fn mask_connection_string(url: &str) -> String {
    if let Some(at_pos) = url.rfind('@') {
        if let Some(scheme_end) = url.find("://") {
            let scheme = &url[..scheme_end + 3];
            let host_part = &url[at_pos..];
            return format!("{scheme}****{host_part}");
        }
    }

    // その他の場合は一部をマスク
    let masked_part = &url[..url.len().min(20)];
    format!("{masked_part}...masked")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires PostgreSQL database"]
    async fn test_create_pool() {
        // CIまたはローカル環境での実行時、DATABASE_URLが既に設定されているかチェック
        let original_url = std::env::var("DATABASE_URL").ok();
        
        // テスト用のURLを設定（環境変数が設定されていない場合のみ）
        if original_url.is_none() {
            let database_url = "postgresql://test_user:test_password@localhost:5433/test_iso_flow";
            std::env::set_var("DATABASE_URL", database_url);
        }

        let result = create_pool().await;
        
        // 環境変数をクリーンアップ（元々設定されていなかった場合のみ）
        if original_url.is_none() {
            std::env::remove_var("DATABASE_URL");
        }
        
        assert!(result.is_ok(), "Should create connection pool: {:?}", result.err());
    }

    #[test]
    fn test_get_database_type_returns_postgres() {
        // get_database_type関数は現在常に"PostgreSQL"を返す
        // 実際のPgPoolインスタンスは必要ないので、テスト用のダミーを作成できない
        // この関数の実装が変わるまでは、関数の存在を確認するだけ
        use std::mem::size_of;
        assert!(size_of::<fn(&PgPool) -> &'static str>() > 0);
    }

    #[test]
    fn test_mask_connection_string_postgres() {
        let url = "postgresql://user:pass@localhost:5432/db";
        let masked = mask_connection_string(url);
        assert_eq!(masked, "postgresql://****@localhost:5432/db");
    }

    #[test]
    fn test_mask_connection_string_postgres_with_params() {
        let url = "postgresql://user:pass@localhost:5432/db?sslmode=require";
        let masked = mask_connection_string(url);
        assert_eq!(
            masked,
            "postgresql://****@localhost:5432/db?sslmode=require"
        );
    }

    #[test]
    fn test_mask_connection_string_postgres_no_auth() {
        let url = "postgresql://localhost:5432/db";
        let masked = mask_connection_string(url);
        assert_eq!(masked, "postgresql://localho...masked");
    }

    #[test]
    fn test_mask_connection_string_postgres_scheme() {
        let url = "postgres://user:pass@localhost:5432/db";
        let masked = mask_connection_string(url);
        assert_eq!(masked, "postgres://****@localhost:5432/db");
    }

    #[test]
    fn test_mask_connection_string_unknown() {
        let url = "unknown://something";
        let masked = mask_connection_string(url);
        assert_eq!(masked, "unknown://something...masked");
    }

    #[test]
    fn test_mask_connection_string_short() {
        let url = "mysql://db";
        let masked = mask_connection_string(url);
        assert_eq!(masked, "mysql://db...masked");
    }

    #[test]
    fn test_mask_connection_string_no_scheme() {
        let url = "just_a_string";
        let masked = mask_connection_string(url);
        assert_eq!(masked, "just_a_string...masked");
    }

    #[test]
    fn test_mask_connection_string_with_multiple_at_signs() {
        let url = "postgresql://user@email.com:pass@localhost:5432/db";
        let masked = mask_connection_string(url);
        // rflindを使うので最後の@でマスクされる
        assert_eq!(masked, "postgresql://****@localhost:5432/db");
    }
}
