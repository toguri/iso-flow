# NBA Trade Tracker - Infrastructure as Code

このディレクトリには、NBA Trade TrackerのAWSインフラストラクチャをTerraformで管理するためのコードが含まれています。

## アーキテクチャ概要

- **フロントエンド**: S3 + CloudFront
- **バックエンド**: ECS Fargate + ALB
- **データベース**: Aurora PostgreSQL Serverless v2
- **スケジューラー**: Amazon MWAA (Managed Apache Airflow)
- **ネットワーク**: VPC with Public/Private/Database subnets

## ディレクトリ構造

```
terraform/
├── environments/          # 環境別の設定
│   ├── dev/              # 開発環境
│   └── prod/             # 本番環境（将来用）
└── modules/              # 再利用可能なモジュール
    ├── vpc/              # ネットワーク構成
    ├── rds/              # Aurora PostgreSQL
    ├── ecs/              # ECS Fargate
    ├── s3/               # S3 + CloudFront
    └── mwaa/             # Amazon MWAA
```

## 前提条件

- Terraform >= 1.5.0
- AWS CLI configured
- 適切なIAM権限

## セットアップ

### 1. 環境変数の設定

```bash
cd terraform/environments/dev
cp terraform.tfvars.example terraform.tfvars
```

`terraform.tfvars`を編集して、必要な値を設定：

```hcl
db_password = "your-secure-password"
```

### 2. バックエンドの設定

`backend.tf`を作成：

```hcl
terraform {
  backend "s3" {
    bucket = "your-terraform-state-bucket"
    key    = "nba-trade-tracker/dev/terraform.tfstate"
    region = "ap-northeast-1"
  }
}
```

### 3. 初期化とデプロイ

```bash
# 初期化
terraform init

# プランの確認
terraform plan

# デプロイ
terraform apply
```

## モジュール詳細

### VPC Module
- 2つのAvailability Zoneにまたがるサブネット構成
- NAT Gatewayによるプライベートサブネットのインターネット接続
- VPC Endpointsによる AWS サービスへのプライベート接続

### RDS Module
- Aurora PostgreSQL Serverless v2
- 自動バックアップとスナップショット
- Secrets Managerによる認証情報管理
- Performance Insights有効

### ECS Module
- Fargate起動タイプ
- Application Load Balancer
- Auto Scaling設定
- CloudWatch Logsへのログ出力

### S3 Module
- フロントエンド用の静的ウェブサイトホスティング
- CloudFrontによるCDN配信
- MWAA DAG用のバージョニング有効化バケット

### MWAA Module
- Apache Airflow 2.8.1
- 最小1、最大2ワーカー（開発環境）
- CloudWatch Logsへの全ログ出力
- Systems Manager Parameter Storeによる接続情報管理

## デプロイ後の作業

### 1. ECRへのイメージプッシュ

```bash
# ECRログイン
aws ecr get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin <ECR_REPOSITORY_URL>

# イメージのビルドとプッシュ
cd ../../../backend
docker build -t nba-trade-tracker .
docker tag nba-trade-tracker:latest <ECR_REPOSITORY_URL>:latest
docker push <ECR_REPOSITORY_URL>:latest
```

### 2. Airflow DAGsのアップロード

```bash
cd ../../../airflow
aws s3 sync dags/ s3://<MWAA_DAG_BUCKET>/dags/
aws s3 cp requirements.txt s3://<MWAA_DAG_BUCKET>/
```

### 3. フロントエンドのデプロイ

```bash
cd ../../../frontend
./deploy.sh
```

## 監視とログ

- CloudWatch Dashboards: 各サービスのメトリクス
- CloudWatch Logs: アプリケーションログ
- X-Ray: 分散トレーシング（将来実装）

## セキュリティ

- すべてのデータは保存時に暗号化
- Secrets Managerによる機密情報管理
- 最小権限の原則に基づくIAMロール
- VPCによるネットワーク分離

## コスト最適化

開発環境では以下の設定でコストを抑えています：
- Aurora Serverless v2: 最小0.5 ACU
- ECS Fargate: 0.25 vCPU, 0.5 GB メモリ
- MWAA: Small環境

## トラブルシューティング

### MWAA環境が起動しない
- VPCに少なくとも2つのプライベートサブネットが必要
- S3バケットのバージョニングが有効になっているか確認

### ECSタスクが起動しない
- ECRにイメージがプッシュされているか確認
- タスクロールに必要な権限があるか確認
- CloudWatch Logsでエラーを確認

## 削除方法

```bash
terraform destroy
```

注意: S3バケットにオブジェクトがある場合は、先に削除する必要があります。