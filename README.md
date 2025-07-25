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
- Apollo GraphQL Client

### バックエンド
- Rust
- Axum (Webフレームワーク)
- async-graphql
- reqwest (HTTPクライアント)

## 📖 ドキュメント

- [開発計画](docs/DEVELOPMENT_PLAN.md)
- [ローカル開発環境セットアップ](docs/LOCAL_DEVELOPMENT.md)

## 🏃 Getting Started

詳細は各ディレクトリのREADMEを参照してください。