# Amazon MWAA Module

Amazon Managed Workflows for Apache Airflow (MWAA) 環境をデプロイするTerraformモジュールです。

## ドキュメント

MWAAの詳細なドキュメントは[docs/airflow/mwaa-setup.md](/docs/airflow/mwaa-setup.md)を参照してください。

## クイックリファレンス

### 基本的な使用例

```hcl
module "mwaa" {
  source = "../../modules/mwaa"
  
  project_name     = "nba-tracker"
  environment      = "dev"
  vpc_id           = module.vpc.vpc_id
  private_subnets  = module.vpc.private_subnet_ids
  
  dag_s3_bucket     = aws_s3_bucket.mwaa.id
  dag_s3_bucket_arn = aws_s3_bucket.mwaa.arn
  
  backend_api_endpoint = module.alb.alb_dns_name
  database_secret_arn  = module.rds.database_secret_arn
}
```

### 主な入力変数

- `project_name` - プロジェクト名
- `environment` - 環境名（dev/staging/prod）
- `vpc_id` - VPCのID
- `private_subnets` - プライベートサブネットのIDリスト（最低2つ）
- `dag_s3_bucket` - DAG用S3バケット名
- `backend_api_endpoint` - バックエンドAPIエンドポイント

### 主な出力値

- `environment_name` - MWAA環境名
- `webserver_url` - Airflow WebサーバーURL
- `execution_role_arn` - MWAA実行ロールARN

## 関連ドキュメント

- [Apache Airflow DAGs Documentation](/docs/airflow/dags.md)
- [MWAA Setup Guide](/docs/airflow/mwaa-setup.md)
- [Terraform Module Source](/terraform/modules/mwaa/)