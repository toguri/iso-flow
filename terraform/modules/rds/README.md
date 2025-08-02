# Aurora PostgreSQL Serverless v2 Module

このモジュールは、AWS Aurora PostgreSQL Serverless v2クラスターをデプロイします。

## ドキュメント

詳細なドキュメントは[docs/terraform/modules/rds.md](/docs/terraform/modules/rds.md)を参照してください。

## クイックリファレンス

### 使用例

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
}
```

### 主な入力変数

- `project_name` - プロジェクト名
- `environment` - 環境名（dev/staging/prod）
- `vpc_id` - VPCのID
- `database_subnets` - データベースサブネットのIDリスト
- `db_name` - データベース名
- `db_username` - マスターユーザー名
- `db_password` - マスターパスワード

詳細な設定オプションは[ドキュメント](/docs/terraform/modules/rds.md)を参照してください。