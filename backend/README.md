# NBA Trade Scraper Backend

[![codecov](https://codecov.io/gh/toguri/iso-flow/branch/main/graph/badge.svg?flag=backend)](https://codecov.io/gh/toguri/iso-flow)

NBAのトレード情報を収集・分析するシステムのバックエンド部分です。

## 現在実装済みの機能

### 1. RSSスクレイピング機能
- **ESPN**と**RealGM**のRSSフィードからNBAニュースを自動取得
- トレード関連キーワードで自動フィルタリング
- ニュースを以下のカテゴリーに自動分類：
  - **Trade**: トレード関連（trade, acquire, deal などのキーワードを含む）
  - **Signing**: 契約・サイン関連（sign, agree, buyout などのキーワードを含む）
  - **Other**: その他のニュース

### 2. GraphQL API
- GraphQL Playgroundで対話的にクエリを実行可能
- 以下のクエリエンドポイントを提供：
  - `tradeNews`: 全てのトレード関連ニュースを取得
  - `tradeNewsByCategory`: カテゴリー別にニュースを取得
  - `tradeNewsBySource`: ソース別にニュースを取得

## セットアップと起動方法

### 1. 依存関係のインストール
```bash
cd backend
cargo build
```

### 2. サーバーの起動
```bash
cargo run
```

サーバーが起動すると、以下のログが表示されます：
```
INFO nba_trade_scraper: Starting GraphQL server...
INFO nba_trade_scraper: GraphQL playground available at http://localhost:3000
```

### 3. GraphQL Playgroundへのアクセス
ブラウザで `http://localhost:3000` にアクセスすると、GraphQL Playgroundが表示されます。

## 使用例

### 全てのトレードニュースを取得
```graphql
query {
  tradeNews {
    id
    title
    link
    source
    publishedAt
    category
  }
}
```

### カテゴリー別にニュースを取得
```graphql
query {
  tradeNewsByCategory(category: "Trade") {
    title
    link
    source
    publishedAt
  }
}
```

### ソース別にニュースを取得
```graphql
query {
  tradeNewsBySource(source: "ESPN") {
    title
    link
    publishedAt
    category
  }
}
```

## テストとカバレッジ

### テストの実行
```bash
# 全テスト実行
make test

# 単体テストのみ
make test-unit

# 統合テストのみ
make test-integration
```

### 統合テスト
以下の統合テストが実装されています：
- **GraphQL API**: エンドポイントの動作確認
- **RSSパーサー**: フィード取得とエラーハンドリング
- **データベース**: CRUD操作と制約の検証

### カバレッジ測定
```bash
# cargo-llvm-covのインストール（初回のみ）
cargo install cargo-llvm-cov

# カバレッジ測定（テキスト形式）
make coverage

# カバレッジ測定（HTML形式）
make coverage-html
# レポートは target/llvm-cov/html/index.html に生成されます
```

## ドキュメント

### APIドキュメントの生成
```bash
# ローカルでドキュメントを生成・表示
make docs

# 依存関係を含むドキュメントを生成
make docs-all
```

### オンラインドキュメント
デプロイされたAPIドキュメントは[こちら](https://toguri.github.io/iso-flow/rust-api/)から参照できます。

## 技術スタック
- **Rust** - プログラミング言語
- **Axum** (v0.8) - Webフレームワーク
- **async-graphql** - GraphQLサーバー実装
- **reqwest** - HTTPクライアント
- **rss** - RSSフィード解析
- **tokio** - 非同期ランタイム
- **SQLx** - データベースライブラリ
- **Sea-ORM** - ORM