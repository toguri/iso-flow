# NBAトレード速報サイト

[![Rust CI](https://github.com/toguri/iso-flow/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/toguri/iso-flow/actions/workflows/rust-ci.yml)

リアルタイムでNBAのトレード情報を表示するWebアプリケーション。

## 🏀 概要

- **フロントエンド**: Kotlin Compose for Web + Apollo GraphQL Client
- **バックエンド**: Rust (Axum + async-graphql)
- **データソース**: RSS フィード / Webスクレイピング

## 🚀 機能

- NBAの最新トレード情報をカード形式で表示
- GraphQLによる効率的なデータ取得
- レスポンシブデザイン対応

## 📁 プロジェクト構成

```
iso-flow/
├── frontend/          # Kotlin Compose for Web
├── backend/           # Rust GraphQL サーバー
├── docs/              # ドキュメント
└── README.md          # このファイル
```

## 🛠️ 技術スタック

### フロントエンド
- Kotlin/JS
- Compose for Web
- Ktor Client（GraphQL通信）

### バックエンド
- Rust
- Axum (Webフレームワーク)
- async-graphql
- SQLx (PostgreSQL)
- reqwest (HTTPクライアント)

### インフラ（本番環境）
- **コンピュート**: ECS Fargate
- **データベース**: RDS PostgreSQL
- **翻訳**: Amazon Translate
- **非同期処理**: Step Functions + Lambda
- **キュー**: Amazon SQS
- **CDN**: CloudFront
- **ロードバランサー**: ALB

### 翻訳処理アーキテクチャ
```
RSSスクレイピング
    ↓
PostgreSQL（translation_status='pending'）
    ↓
SQS（新規記事通知）
    ↓
Step Functions（ワークフロー管理）
    ↓
Lambda（翻訳実行）
    ↓
Amazon Translate
    ↓
PostgreSQL（translation_status='completed'）
```

## 📖 ドキュメント

- [開発計画](docs/DEVELOPMENT_PLAN.md)
- [ローカル開発環境セットアップ](docs/LOCAL_DEVELOPMENT.md)

## 🏃 Getting Started

詳細は各ディレクトリのREADMEを参照してください。