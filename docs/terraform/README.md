# Terraform Documentation

このディレクトリには、NBA Trade TrackerプロジェクトのTerraform関連のドキュメントが含まれています。

## ドキュメント一覧

1. [インフラストラクチャ概要](./infrastructure-overview.md) - AWS構成とアーキテクチャ
2. [セットアップガイド](./setup-guide.md) - Terraformの初期設定とデプロイ手順
3. [モジュールリファレンス](./modules/) - 各Terraformモジュールの詳細
   - [VPC Module](./modules/vpc.md)
   - [RDS Module](./modules/rds.md)
   - [ECS Module](./modules/ecs.md)
   - [S3 Module](./modules/s3.md)
   - [MWAA Module](./modules/mwaa.md)
4. [運用ガイド](./operations.md) - 監視、トラブルシューティング、コスト最適化

## アーキテクチャ概要

```
┌─────────────────────────────────────────────────────────────────┐
│                         CloudFront                               │
└─────────────────────────────┬───────────────────────────────────┘
                              │
┌─────────────────────────────┴───────────────────────────────────┐
│                           S3 Bucket                              │
│                    (Frontend Static Files)                       │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                            ALB                                   │
└─────────────────────────────┬───────────────────────────────────┘
                              │
┌─────────────────────────────┴───────────────────────────────────┐
│                         ECS Fargate                              │
│                     (Backend API - Rust)                         │
└─────────────────────────────┬───────────────────────────────────┘
                              │
        ┌─────────────────────┴─────────────────────┐
        │                                           │
┌───────┴──────────┐                    ┌──────────┴───────────┐
│  Aurora RDS      │                    │      MWAA            │
│  PostgreSQL      │◀───────────────────│  (Apache Airflow)    │
└──────────────────┘                    └──────────────────────┘
```

## 使用技術

- **IaC**: Terraform v1.5+
- **AWS Services**:
  - VPC (Multi-AZ)
  - Aurora PostgreSQL Serverless v2
  - ECS Fargate
  - Application Load Balancer
  - S3 + CloudFront
  - Amazon MWAA
  - Secrets Manager
  - Systems Manager Parameter Store

## クイックスタート

```bash
# 開発環境へのデプロイ
cd terraform/environments/dev
terraform init
terraform plan
terraform apply
```

詳細は[セットアップガイド](./setup-guide.md)を参照してください。

## プロジェクト構造

```
terraform/
├── environments/      # 環境別設定
│   ├── dev/          # 開発環境
│   └── prod/         # 本番環境（将来用）
└── modules/          # 再利用可能なモジュール
    ├── vpc/          # ネットワーク構成
    ├── rds/          # Aurora PostgreSQL
    ├── ecs/          # ECS Fargate
    ├── s3/           # S3 + CloudFront
    └── mwaa/         # Amazon MWAA
```

## 環境管理

| 環境 | 用途 | 特徴 |
|-----|------|------|
| dev | 開発・テスト | コスト最適化設定、最小構成 |
| staging | ステージング（計画中） | 本番に近い構成 |
| prod | 本番（計画中） | 高可用性、自動スケーリング |

## セキュリティ

- すべてのデータは保存時に暗号化
- Secrets Managerによる機密情報管理
- 最小権限の原則に基づくIAMロール
- VPCによるネットワーク分離
- セキュリティグループによるアクセス制御

## 関連リンク

- [AWS Terraform Provider Documentation](https://registry.terraform.io/providers/hashicorp/aws/latest/docs)
- [Terraform Best Practices](https://www.terraform.io/docs/cloud/guides/recommended-practices/index.html)
- [AWS Well-Architected Framework](https://aws.amazon.com/architecture/well-architected/)