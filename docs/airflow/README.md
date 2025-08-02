# Apache Airflow Documentation

このディレクトリには、NBA Trade TrackerプロジェクトのApache Airflow関連のドキュメントが含まれています。

## ドキュメント一覧

1. [Apache Airflow DAGs](./dags.md) - DAGの実装と運用方法
2. [Amazon MWAA環境構築](./mwaa-setup.md) - Terraformを使用したMWAA環境の構築方法

## 概要

NBA Trade Trackerでは、Amazon MWAA (Managed Workflows for Apache Airflow) を使用して、定期的なRSSフィードのスクレイピングを自動化しています。

### 主な機能

- 5分間隔でのRSSフィードスクレイピング
- GraphQL APIを通じたデータの取得と保存
- CloudWatch Logsによる包括的なロギング
- 自動スケーリング機能（1〜2ワーカー）

### アーキテクチャ

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│    MWAA     │────▶│  GraphQL    │────▶│  PostgreSQL │
│  (Airflow)  │     │    API      │     │   Database  │
└─────────────┘     └─────────────┘     └─────────────┘
       │                                         ▲
       │                                         │
       └─────────────────────────────────────────┘
                  (Direct DB Access)
```

## クイックスタート

1. Terraform環境の準備（[詳細](./mwaa-setup.md)を参照）
2. DAGのデプロイ（[詳細](./dags.md#dagのデプロイ)を参照）
3. MWAA Webserver UIでDAGの実行状態を確認

## 関連リンク

- [Terraform MWAA Module](/terraform/modules/mwaa/)
- [Airflow DAGs実装](/airflow/dags/)
- [バックエンドAPI GraphQLスキーマ](/backend/src/graphql.rs)