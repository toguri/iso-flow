# Aurora PostgreSQL Serverless v2 Module

このモジュールは、AWS Aurora PostgreSQL Serverless v2クラスターをデプロイします。

## 特徴

- Aurora Serverless v2による自動スケーリング（0.5〜1.0 ACU）
- PostgreSQL 15.4対応
- 自動バックアップ（7日間保持）
- Performance Insights有効
- Secrets Managerによる認証情報管理
- VPC内でのセキュアな接続

## 使用方法

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
  }
}
```

## 入力変数

| 名前 | 説明 | タイプ | 必須 |
|------|------|--------|------|
| project_name | プロジェクト名 | string | yes |
| environment | 環境名（dev/staging/prod） | string | yes |
| vpc_id | VPCのID | string | yes |
| database_subnets | データベースサブネットのIDリスト | list(string) | yes |
| db_name | データベース名 | string | yes |
| db_username | マスターユーザー名 | string | yes |
| db_password | マスターパスワード | string | yes |
| tags | リソースタグ | map(string) | no |

## 出力値

| 名前 | 説明 |
|------|------|
| cluster_endpoint | Auroraクラスターのエンドポイント |
| cluster_reader_endpoint | 読み取り専用エンドポイント |
| cluster_port | データベースポート（5432） |
| database_name | データベース名 |
| database_url | PostgreSQL接続URL |
| security_group_id | セキュリティグループID |
| database_secret_arn | Secrets ManagerシークレットのARN |

## セキュリティ考慮事項

1. **ネットワーク分離**: データベースはプライベートサブネットに配置
2. **暗号化**: 保存時の暗号化が有効
3. **認証情報管理**: Secrets Managerで安全に管理
4. **バックアップ**: 自動バックアップで障害に備える

## コスト最適化

- Serverless v2により、使用率に応じた自動スケーリング
- 最小0.5 ACUから開始し、負荷に応じて最大1.0 ACUまで拡張
- 開発環境では最終スナップショットをスキップしてコスト削減

## 移行ガイド

SQLiteからの移行は、付属の移行スクリプトを使用：

```bash
cd backend/scripts
export DATABASE_URL="postgresql://..."
export SQLITE_DB_PATH="./nba_trades.db"
./migrate_to_aurora.sh
```