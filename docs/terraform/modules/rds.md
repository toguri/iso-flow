# Aurora PostgreSQL Serverless v2 Module

AWS Aurora PostgreSQL Serverless v2クラスターをデプロイするTerraformモジュールです。

## 概要

このモジュールは、高可用性とスケーラビリティを備えたマネージドPostgreSQLデータベースを提供します。

### 主な特徴

- Aurora Serverless v2による自動スケーリング（0.5〜1.0 ACU）
- PostgreSQL 15.4対応
- Multi-AZ構成による高可用性
- 自動バックアップ（7日間保持）
- Performance Insights有効
- Secrets Managerによる認証情報管理
- 保存時の暗号化

## モジュールの使用方法

### 基本的な使用例

```hcl
module "rds" {
  source = "../../modules/rds"
  
  project_name     = "nba-tracker"
  environment      = "dev"
  vpc_id           = module.vpc.vpc_id
  database_subnets = module.vpc.database_subnet_ids
  
  db_name     = "nbatracker"
  db_username = "nbatracker"
  db_password = var.database_password
  
  tags = {
    Component = "Database"
    Owner     = "Backend Team"
  }
}
```

### 高度な設定例

```hcl
module "rds" {
  source = "../../modules/rds"
  
  project_name     = "nba-tracker"
  environment      = "prod"
  vpc_id           = module.vpc.vpc_id
  database_subnets = module.vpc.database_subnet_ids
  
  # データベース設定
  db_name     = "nbatracker"
  db_username = "nbatracker"
  db_password = var.database_password
  
  # スケーリング設定
  min_capacity = 0.5
  max_capacity = 4.0
  
  # バックアップ設定
  backup_retention_period = 30
  backup_window          = "03:00-04:00"
  maintenance_window     = "sun:04:00-sun:05:00"
  
  # Performance Insights
  performance_insights_retention_period = 7
  
  # 削除保護
  deletion_protection = true
  skip_final_snapshot = false
  
  tags = {
    Component   = "Database"
    Environment = "Production"
    CostCenter  = "Engineering"
  }
}
```

## 入力変数

### 必須変数

| 名前 | 説明 | タイプ | 例 |
|------|------|--------|-----|
| `project_name` | プロジェクト名（リソース名のプレフィックス） | string | "nba-tracker" |
| `environment` | 環境名（dev/staging/prod） | string | "dev" |
| `vpc_id` | VPCのID | string | "vpc-12345678" |
| `database_subnets` | データベースサブネットのIDリスト（最低2つ） | list(string) | ["subnet-1a", "subnet-1b"] |
| `db_name` | 作成するデータベース名 | string | "nbatracker" |
| `db_username` | マスターユーザー名 | string | "nbatracker" |
| `db_password` | マスターパスワード（8文字以上） | string | "SecurePass123!" |

### オプション変数

| 名前 | 説明 | タイプ | デフォルト |
|------|------|--------|-----------|
| `engine_version` | PostgreSQLのバージョン | string | "15.4" |
| `instance_class` | インスタンスクラス | string | "db.serverless" |
| `min_capacity` | 最小ACU | number | 0.5 |
| `max_capacity` | 最大ACU | number | 1.0 |
| `backup_retention_period` | バックアップ保持期間（日） | number | 7 |
| `backup_window` | バックアップウィンドウ | string | "03:00-04:00" |
| `maintenance_window` | メンテナンスウィンドウ | string | "sun:04:00-sun:05:00" |
| `deletion_protection` | 削除保護の有効化 | bool | false |
| `skip_final_snapshot` | 最終スナップショットのスキップ | bool | true |
| `performance_insights_retention_period` | Performance Insights保持期間（日） | number | 7 |
| `tags` | リソースタグ | map(string) | {} |

## 出力値

| 名前 | 説明 | 使用例 |
|------|------|--------|
| `cluster_endpoint` | Auroraクラスターのエンドポイント | 書き込み用接続 |
| `cluster_reader_endpoint` | 読み取り専用エンドポイント | 読み取り負荷分散 |
| `cluster_port` | データベースポート | 5432 |
| `database_name` | データベース名 | アプリケーション設定 |
| `database_url` | PostgreSQL接続URL | `DATABASE_URL`環境変数 |
| `security_group_id` | セキュリティグループID | 追加ルール設定 |
| `database_secret_arn` | Secrets ManagerシークレットのARN | IAMポリシー設定 |

## セキュリティ

### ネットワークセキュリティ

```hcl
# セキュリティグループのルール
resource "aws_security_group_rule" "app_to_rds" {
  type                     = "ingress"
  from_port                = 5432
  to_port                  = 5432
  protocol                 = "tcp"
  source_security_group_id = aws_security_group.app.id
  security_group_id        = module.rds.security_group_id
}
```

### 暗号化

- **保存時の暗号化**: デフォルトで有効（AWS管理のKMS鍵）
- **転送時の暗号化**: SSL/TLS接続を推奨

```bash
# SSL接続の強制
psql "postgresql://username:password@endpoint:5432/dbname?sslmode=require"
```

### 認証情報管理

Secrets Managerで管理される認証情報の取得：

```bash
# AWS CLIでの取得
aws secretsmanager get-secret-value \
  --secret-id $(terraform output -raw database_secret_arn) \
  --query SecretString \
  --output text | jq -r .password
```

アプリケーションでの使用：

```rust
// Rust example
use aws_sdk_secretsmanager::Client;

async fn get_db_password() -> Result<String, Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    
    let secret = client
        .get_secret_value()
        .secret_id(std::env::var("DATABASE_SECRET_ARN")?)
        .send()
        .await?;
    
    let secret_string = secret.secret_string().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(secret_string)?;
    Ok(parsed["password"].as_str().unwrap().to_string())
}
```

## 運用

### バックアップとリストア

#### 手動スナップショットの作成

```bash
aws rds create-db-cluster-snapshot \
  --db-cluster-snapshot-identifier manual-snapshot-$(date +%Y%m%d) \
  --db-cluster-identifier $(terraform output -raw cluster_id)
```

#### スナップショットからのリストア

```bash
aws rds restore-db-cluster-from-snapshot \
  --db-cluster-identifier restored-cluster \
  --snapshot-identifier manual-snapshot-20240101 \
  --engine aurora-postgresql
```

### スケーリング

#### 手動スケーリング

```bash
# ACU範囲の変更
aws rds modify-db-cluster \
  --db-cluster-identifier $(terraform output -raw cluster_id) \
  --serverless-v2-scaling-configuration MinCapacity=1.0,MaxCapacity=4.0 \
  --apply-immediately
```

#### 自動スケーリングの動作

Aurora Serverless v2は以下の条件で自動的にスケーリングします：

1. **スケールアップ条件**
   - CPU使用率が高い
   - 接続数が増加
   - メモリ使用率が高い

2. **スケールダウン条件**
   - リソース使用率が低下して15分経過
   - アクティブな接続がない

### モニタリング

#### 主要メトリクス

```bash
# CloudWatch メトリクスの取得
aws cloudwatch get-metric-statistics \
  --namespace AWS/RDS \
  --metric-name ServerlessDatabaseCapacity \
  --dimensions Name=DBClusterIdentifier,Value=$(terraform output -raw cluster_id) \
  --start-time $(date -u -d '1 hour ago' +%Y-%m-%dT%H:%M:%S) \
  --end-time $(date -u +%Y-%m-%dT%H:%M:%S) \
  --period 300 \
  --statistics Average
```

#### Performance Insights

```bash
# パフォーマンスメトリクスの確認
aws pi get-resource-metrics \
  --service-type RDS \
  --identifier $(terraform output -raw cluster_resource_id) \
  --metric-queries file://metrics-query.json \
  --start-time $(date -u -d '1 hour ago' +%Y-%m-%dT%H:%M:%S) \
  --end-time $(date -u +%Y-%m-%dT%H:%M:%S)
```

### トラブルシューティング

#### 接続できない場合

1. **セキュリティグループの確認**
   ```bash
   aws ec2 describe-security-groups \
     --group-ids $(terraform output -raw security_group_id)
   ```

2. **クラスターステータスの確認**
   ```bash
   aws rds describe-db-clusters \
     --db-cluster-identifier $(terraform output -raw cluster_id)
   ```

3. **ネットワーク接続性のテスト**
   ```bash
   telnet $(terraform output -raw cluster_endpoint) 5432
   ```

#### パフォーマンス問題

1. **現在のACU使用状況**
   ```bash
   aws cloudwatch get-metric-statistics \
     --namespace AWS/RDS \
     --metric-name ServerlessDatabaseCapacity \
     --dimensions Name=DBClusterIdentifier,Value=$(terraform output -raw cluster_id) \
     --start-time $(date -u -d '10 minutes ago' +%Y-%m-%dT%H:%M:%S) \
     --end-time $(date -u +%Y-%m-%dT%H:%M:%S) \
     --period 60 \
     --statistics Maximum
   ```

2. **クエリパフォーマンスの分析**
   - Performance Insightsでスロークエリを特定
   - EXPLAIN ANALYZEでクエリプランを確認
   - 必要に応じてインデックスを追加

## コスト最適化

### ACU使用量の最適化

1. **使用パターンの分析**
   - CloudWatch メトリクスで使用傾向を確認
   - ピーク時間とアイドル時間を特定

2. **適切なmin/max設定**
   - 開発環境: 0.5-1.0 ACU
   - ステージング環境: 0.5-2.0 ACU
   - 本番環境: 1.0-4.0 ACU（負荷に応じて調整）

3. **接続プーリングの使用**
   - アプリケーション側で接続プールを実装
   - 不要な接続を迅速に解放

### バックアップコストの削減

```hcl
# 開発環境では短い保持期間
backup_retention_period = var.environment == "dev" ? 1 : 7

# 開発環境では最終スナップショットをスキップ
skip_final_snapshot = var.environment == "dev" ? true : false
```

## 移行ガイド

### 既存データベースからの移行

1. **pg_dumpを使用したエクスポート**
   ```bash
   pg_dump -h old-host -U username -d dbname > backup.sql
   ```

2. **新しいクラスターへのインポート**
   ```bash
   psql -h $(terraform output -raw cluster_endpoint) \
        -U $(terraform output -raw master_username) \
        -d $(terraform output -raw database_name) \
        < backup.sql
   ```

3. **データ検証**
   ```sql
   -- レコード数の確認
   SELECT schemaname, tablename, n_live_tup 
   FROM pg_stat_user_tables 
   ORDER BY n_live_tup DESC;
   ```

## ベストプラクティス

1. **接続管理**
   - 接続プールの使用を推奨
   - アイドル接続のタイムアウト設定
   - 接続数の監視

2. **セキュリティ**
   - 最小権限の原則に従ったユーザー作成
   - 定期的なパスワードローテーション
   - 監査ログの有効化

3. **パフォーマンス**
   - 適切なインデックスの作成
   - 定期的なVACUUM/ANALYZE
   - クエリ最適化

4. **可用性**
   - 複数AZでの読み取りレプリカ
   - 自動フェイルオーバーのテスト
   - 定期的なリストア訓練