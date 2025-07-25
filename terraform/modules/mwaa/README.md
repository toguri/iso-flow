# Amazon MWAA Module

このモジュールは、Amazon Managed Workflows for Apache Airflow (MWAA) 環境をデプロイします。

## 特徴

- Apache Airflow 2.8.1対応
- 自動スケーリング（1〜2ワーカー）
- CloudWatch Logsによる包括的なロギング
- VPC内でのセキュアな実行
- SSM Parameter Storeによる接続情報管理

## 使用方法

```hcl
module "mwaa" {
  source = "../../modules/mwaa"
  
  project_name     = "nba-tracker"
  environment      = "dev"
  vpc_id           = module.vpc.vpc_id
  private_subnets  = module.vpc.private_subnet_ids
  
  dag_s3_bucket     = aws_s3_bucket.mwaa.id
  dag_s3_bucket_arn = aws_s3_bucket.mwaa.arn
  
  environment_class = "mw1.small"
  min_workers       = 1
  max_workers       = 2
  
  backend_api_endpoint = "http://alb-dns-name"
  database_secret_arn  = module.rds.database_secret_arn
  
  tags = {
    Component = "Scheduler"
  }
}
```

## 入力変数

| 名前 | 説明 | タイプ | 必須 |
|------|------|--------|------|
| project_name | プロジェクト名 | string | yes |
| environment | 環境名（dev/staging/prod） | string | yes |
| vpc_id | VPCのID | string | yes |
| private_subnets | プライベートサブネットのIDリスト（最低2つ） | list(string) | yes |
| dag_s3_bucket | DAG用S3バケット名 | string | yes |
| dag_s3_bucket_arn | DAG用S3バケットARN | string | yes |
| environment_class | MWAA環境クラス | string | no |
| min_workers | 最小ワーカー数 | number | no |
| max_workers | 最大ワーカー数 | number | no |
| backend_api_endpoint | バックエンドAPIエンドポイント | string | yes |
| database_secret_arn | データベースシークレットARN | string | yes |
| tags | リソースタグ | map(string) | no |

## 出力値

| 名前 | 説明 |
|------|------|
| environment_name | MWAA環境名 |
| environment_arn | MWAA環境ARN |
| webserver_url | Airflow WebサーバーURL |
| execution_role_arn | MWAA実行ロールARN |
| results_bucket_name | 結果保存用S3バケット名 |
| security_group_id | MWAAセキュリティグループID |

## DAGの配置

DAGファイルは以下の構造でS3バケットに配置します：

```
s3://bucket-name/
├── dags/
│   └── nba_rss_scraper.py
└── requirements.txt
```

## セキュリティ考慮事項

1. **ネットワーク分離**: MWAAはプライベートサブネットで実行
2. **IAMロール**: 最小権限の原則に基づいたロール設定
3. **シークレット管理**: データベース認証情報はSecrets Manager経由
4. **ログ保護**: CloudWatch Logsで暗号化

## コスト最適化

- mw1.smallクラスで開始（最小構成）
- ワーカー数は負荷に応じて自動スケーリング
- 不要な時はenable_mwaa=falseで無効化可能

## トラブルシューティング

### DAGが表示されない
- S3バケットのバージョニングが有効か確認
- DAGファイルの構文エラーをチェック
- CloudWatch Logs（DAGProcessing）でエラーを確認

### タスクが実行されない
- ワーカーログを確認
- ネットワーク接続（バックエンドAPI、RDS）を確認
- IAMロールの権限を確認

### 環境の起動に失敗
- VPCに最低2つのプライベートサブネットがあるか確認
- セキュリティグループのルールを確認
- S3バケットへのアクセス権限を確認