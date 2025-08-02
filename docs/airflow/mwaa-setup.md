# Amazon MWAA環境構築ガイド

Amazon Managed Workflows for Apache Airflow (MWAA) 環境の構築と設定に関するドキュメントです。

## 概要

MWAAは、Apache Airflowのマネージドサービスで、以下の特徴があります：

- Apache Airflow 2.8.1対応
- 自動スケーリング（1〜2ワーカー）
- CloudWatch Logsによる包括的なロギング
- VPC内でのセキュアな実行
- SSM Parameter Storeによる接続情報管理

## アーキテクチャ

```
┌─────────────────────────────────────────────┐
│                    VPC                       │
├─────────────────────┬───────────────────────┤
│  Private Subnet 1   │   Private Subnet 2    │
│                     │                        │
│   ┌─────────────┐   │   ┌─────────────┐    │
│   │    MWAA     │   │   │    MWAA     │    │
│   │  Worker 1   │   │   │  Worker 2   │    │
│   └─────────────┘   │   └─────────────┘    │
│          │          │          │             │
│          └──────────┴──────────┘             │
│                     │                        │
│         ┌───────────┴───────────┐            │
│         │   Security Group      │            │
│         │  - Ingress: 443       │            │
│         │  - Egress: All        │            │
│         └───────────────────────┘            │
└──────────────────────────────────────────────┘
                      │
              ┌───────┴───────┐
              │  S3 Bucket    │
              │  - DAGs       │
              │  - Logs       │
              └───────────────┘
```

## Terraform Moduleの使用方法

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

### 入力変数

| 名前 | 説明 | タイプ | デフォルト | 必須 |
|------|------|--------|-----------|------|
| project_name | プロジェクト名 | string | - | yes |
| environment | 環境名（dev/staging/prod） | string | - | yes |
| vpc_id | VPCのID | string | - | yes |
| private_subnets | プライベートサブネットのIDリスト（最低2つ） | list(string) | - | yes |
| dag_s3_bucket | DAG用S3バケット名 | string | - | yes |
| dag_s3_bucket_arn | DAG用S3バケットARN | string | - | yes |
| environment_class | MWAA環境クラス | string | "mw1.small" | no |
| min_workers | 最小ワーカー数 | number | 1 | no |
| max_workers | 最大ワーカー数 | number | 2 | no |
| backend_api_endpoint | バックエンドAPIエンドポイント | string | - | yes |
| database_secret_arn | データベースシークレットARN | string | - | yes |
| tags | リソースタグ | map(string) | {} | no |

### 出力値

| 名前 | 説明 |
|------|------|
| environment_name | MWAA環境名 |
| environment_arn | MWAA環境ARN |
| webserver_url | Airflow WebサーバーURL |
| execution_role_arn | MWAA実行ロールARN |
| results_bucket_name | 結果保存用S3バケット名 |
| security_group_id | MWAAセキュリティグループID |

## セットアップ手順

### 1. 前提条件

- AWS CLIがインストールされ、認証設定済み
- Terraformがインストール済み（v1.0以上）
- VPCとプライベートサブネット（最低2つ）が作成済み

### 2. S3バケットの準備

```bash
# S3バケットの作成（Terraform外で作成する場合）
aws s3 mb s3://your-mwaa-bucket-name

# バージョニングの有効化（必須）
aws s3api put-bucket-versioning \
  --bucket your-mwaa-bucket-name \
  --versioning-configuration Status=Enabled
```

### 3. DAGの配置

DAGファイルは以下の構造でS3バケットに配置します：

```
s3://bucket-name/
├── dags/
│   └── nba_rss_scraper.py
└── requirements.txt
```

### 4. Terraformの実行

```bash
# 環境ディレクトリに移動
cd terraform/environments/dev

# 初期化
terraform init

# プランの確認
terraform plan -var="enable_mwaa=true"

# 適用
terraform apply -var="enable_mwaa=true"
```

### 5. 環境変数の設定

Terraformが自動的に設定しますが、手動で追加する場合：

```bash
# バックエンドAPIエンドポイント
aws ssm put-parameter \
  --name "/airflow/variables/backend_endpoint" \
  --value "http://your-alb-dns/graphql" \
  --type String

# その他の変数
aws ssm put-parameter \
  --name "/airflow/variables/your_variable" \
  --value "your_value" \
  --type String
```

## セキュリティ考慮事項

### 1. ネットワーク分離

- MWAAはプライベートサブネットで実行
- インターネットアクセスはNATゲートウェイ経由
- セキュリティグループで必要最小限のアクセスのみ許可

### 2. IAMロール

最小権限の原則に基づいたロール設定：

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "s3:GetObject",
        "s3:ListBucket"
      ],
      "Resource": [
        "arn:aws:s3:::your-mwaa-bucket/*",
        "arn:aws:s3:::your-mwaa-bucket"
      ]
    },
    {
      "Effect": "Allow",
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "arn:aws:logs:*:*:*"
    }
  ]
}
```

### 3. シークレット管理

- データベース認証情報はSecrets Manager経由
- APIキーなどの機密情報もSecrets Managerで管理
- 環境変数にハードコードしない

### 4. ログ保護

- CloudWatch Logsで暗号化
- ログの保持期間を適切に設定
- 機密情報がログに出力されないよう注意

## コスト最適化

### 環境クラスの選択

| クラス | CPU | メモリ | 推奨用途 |
|--------|-----|--------|----------|
| mw1.small | 1 vCPU | 2 GB | 開発/テスト環境 |
| mw1.medium | 2 vCPU | 4 GB | 小規模本番環境 |
| mw1.large | 4 vCPU | 8 GB | 中規模本番環境 |

### コスト削減のヒント

1. **開発環境はmw1.smallで開始**
   - 必要に応じてスケールアップ

2. **ワーカー数の最適化**
   - 負荷に応じて自動スケーリング設定
   - 夜間や週末は最小構成に

3. **不要時の環境停止**
   ```bash
   terraform apply -var="enable_mwaa=false"
   ```

4. **ログの保持期間**
   - 必要最小限の期間に設定
   - 古いログはS3にアーカイブ

## トラブルシューティング

### DAGが表示されない

1. **S3バケットのバージョニングが有効か確認**
   ```bash
   aws s3api get-bucket-versioning --bucket your-mwaa-bucket
   ```

2. **DAGファイルの構文エラーをチェック**
   ```bash
   python dags/your_dag.py
   ```

3. **CloudWatch Logs（DAGProcessing）でエラーを確認**
   ```bash
   aws logs tail /aws/mwaa/environment-name/DAGProcessing
   ```

### タスクが実行されない

1. **ワーカーログを確認**
   ```bash
   aws logs tail /aws/mwaa/environment-name/Worker
   ```

2. **ネットワーク接続を確認**
   - セキュリティグループ
   - NACLs
   - ルートテーブル

3. **IAMロールの権限を確認**
   - 必要なAWSサービスへのアクセス権限

### 環境の起動に失敗

1. **VPCに最低2つのプライベートサブネットがあるか確認**
2. **セキュリティグループのルールを確認**
3. **S3バケットへのアクセス権限を確認**

### パフォーマンス問題

1. **ワーカー数の確認**
   - CloudWatchメトリクスでCPU/メモリ使用率を確認

2. **タスクの並列度**
   - DAGの並列実行設定を最適化

3. **環境クラスのアップグレード**
   - 必要に応じてより大きなインスタンスクラスに

## メンテナンス

### 定期的なタスク

1. **ログのローテーション**
   - CloudWatch Logsの保持期間設定
   - 古いログのアーカイブ

2. **DAGのクリーンアップ**
   - 使用されていないDAGの削除
   - 古いDAGバージョンの削除

3. **セキュリティアップデート**
   - Airflowバージョンのアップデート
   - Pythonパッケージの更新

### バックアップ

1. **DAGファイル**
   - S3バージョニングで自動バックアップ
   - 定期的なS3バックアップ

2. **メタデータ**
   - MWAAが自動的に管理
   - 必要に応じてエクスポート

## 参考リンク

- [AWS MWAA公式ドキュメント](https://docs.aws.amazon.com/mwaa/)
- [Apache Airflow公式ドキュメント](https://airflow.apache.org/docs/)
- [Terraform AWS Provider](https://registry.terraform.io/providers/hashicorp/aws/latest/docs)