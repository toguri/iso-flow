# Amazon MWAA Module

このモジュールは、Amazon Managed Workflows for Apache Airflow (MWAA) 環境をデプロイします。

## ドキュメント

詳細なドキュメントは[docs/airflow/mwaa-setup.md](/docs/airflow/mwaa-setup.md)を参照してください。

## クイックリファレンス

### 使用例

```hcl
module "mwaa" {
  source = "../../modules/mwaa"
  
  project_name     = "nba-tracker"
  environment      = "dev"
  vpc_id           = module.vpc.vpc_id
  private_subnets  = module.vpc.private_subnet_ids
  
  dag_s3_bucket     = aws_s3_bucket.mwaa.id
  dag_s3_bucket_arn = aws_s3_bucket.mwaa.arn
  
  backend_api_endpoint = "http://alb-dns-name"
  database_secret_arn  = module.rds.database_secret_arn
}
```

### 主な入力変数

- `project_name` - プロジェクト名
- `environment` - 環境名（dev/staging/prod）
- `vpc_id` - VPCのID
- `private_subnets` - プライベートサブネットのIDリスト

詳細な設定オプションは[ドキュメント](/docs/airflow/mwaa-setup.md#terraformmoduleの使用方法)を参照してください。