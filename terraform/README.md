# NBA Trade Tracker - Infrastructure as Code

このディレクトリには、NBA Trade TrackerのAWSインフラストラクチャをTerraformで管理するためのコードが含まれています。

## ドキュメント

詳細なドキュメントは[docs/terraform/](/docs/terraform/)を参照してください：

- [インフラストラクチャ概要](/docs/terraform/infrastructure-overview.md) - AWS構成とアーキテクチャ
- [セットアップガイド](/docs/terraform/setup-guide.md) - Terraformの初期設定とデプロイ手順
- [モジュールリファレンス](/docs/terraform/modules/) - 各Terraformモジュールの詳細
- [運用ガイド](/docs/terraform/operations.md) - 監視、トラブルシューティング、コスト最適化

## ディレクトリ構造

```
terraform/
├── README.md              # このファイル
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

## クイックスタート

```bash
# 開発環境へのデプロイ
cd environments/dev
terraform init
terraform plan
terraform apply
```

詳細な手順は[セットアップガイド](/docs/terraform/setup-guide.md)を参照してください。

## アーキテクチャ概要

- **フロントエンド**: S3 + CloudFront
- **バックエンド**: ECS Fargate + ALB
- **データベース**: Aurora PostgreSQL Serverless v2
- **スケジューラー**: Amazon MWAA (Managed Apache Airflow)
- **ネットワーク**: VPC with Public/Private/Database subnets

詳細は[インフラストラクチャ概要](/docs/terraform/infrastructure-overview.md)を参照してください。