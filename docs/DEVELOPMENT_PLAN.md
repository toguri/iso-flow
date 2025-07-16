# 開発計画書

## 🎯 プロジェクトゴール

「NBAトレード速報を一覧表示するサイト」をCompose for Web + Rust(GraphQL)構成で完成させる

## 📚 習得スキル

| 項目 | 得られるスキル・成果 |
|------|---------------------|
| Compose for Web | KotlinでHTML/CSSを構築し、SPAを構成できる |
| Apollo Kotlin Client | GraphQLクエリの定義と型安全なデータ取得 |
| Rust (Axum + async-graphql) | GraphQLサーバー構築と外部データ取得処理 |
| スクレイピング | RSS or Webデータの非同期取得 + パース処理 |
| システム構成 | Rust ↔ Kotlin間のフロント／バック統合スキル |
| リリース経験 | 静的サイトとAPIサーバーを別々にデプロイする構成理解 |

## 🏗️ アーキテクチャ

```
┌─────────────────────────┐
│   Compose for Web (SPA) │ ← Kotlin + Apollo Client
│   - カード形式のUI     │
│   - GraphQLクエリ発行   │
└───────────┬─────────────┘
            │ GraphQL
┌───────────▼─────────────┐
│   Rust Backend          │ ← Axum + async-graphql
│   - /graphql エンドポイント │
│   - RSSフィード取得     │
│   - データパース処理    │
└─────────────────────────┘
```

### フロントエンド仕様
- **フレームワーク**: Kotlin Compose for Web
- **通信**: Apollo GraphQL Client
- **UI**: トレード情報をカード形式で表示
- **ビルド**: Kotlin/JS → 静的ファイル生成

### バックエンド仕様
- **言語**: Rust
- **Webフレームワーク**: Axum
- **GraphQL**: async-graphql
- **データ取得**: reqwest (非同期HTTPクライアント)
- **エンドポイント**: `/graphql`

## 📱 画面イメージ

```
📦 トレード速報画面

──────────────────────────────
📅 2025-07-14
🏀 Lakers acquire Dejounte Murray from Hawks
    詳細: Rui Hachimura, draft picks sent to ATL
──────────────────────────────
📅 2025-07-13
🏀 Hornets sign-and-trade Miles Bridges to Magic
──────────────────────────────
```

## 📋 開発手順

### Phase 1: バックエンド構築
1. **Rustプロジェクト作成**
   - Cargoプロジェクト初期化
   - 依存関係設定（Axum, async-graphql, tokio等）

2. **GraphQLサーバー基本実装**
   - GraphQLスキーマ定義
   - モックデータでのクエリ実装
   - `/graphql` エンドポイント設定

3. **データ取得機能**
   - RSS/Webスクレイピング実装
   - データパース処理
   - エラーハンドリング

### Phase 2: フロントエンド構築
1. **Kotlinプロジェクト作成**
   - Compose for Webプロジェクト初期化
   - Apollo Client設定

2. **GraphQL連携**
   - スキーマ取得と型生成
   - クエリ定義
   - データフェッチ実装

3. **UI実装**
   - カード形式のコンポーネント作成
   - レスポンシブデザイン対応
   - ローディング/エラー表示

### Phase 3: 統合とデプロイ
1. **ローカル統合テスト**
   - CORS設定
   - エンドポイント接続確認

2. **デプロイ準備**
   - フロントエンド: 静的ファイルビルド
   - バックエンド: Dockerイメージ作成

3. **本番デプロイ**
   - フロントエンド: Vercel/Netlify等
   - バックエンド: Fly.io/Railway等

## 🔧 必要な開発環境

- **Rust**: 最新安定版
- **JDK**: 11以上
- **Node.js**: 16以上
- **Git**

## 📊 進捗管理

各フェーズごとに以下を確認：
- [ ] 実装完了
- [ ] 動作確認
- [ ] ドキュメント更新

## 🌐 データソース候補

- HoopsHype RSS フィード
- ESPN Trade Tracker
- The Athletic
- Woj/Shams Twitter

## ⚠️ 注意事項

- データ取得時のレート制限に配慮
- 著作権に配慮したデータ利用
- エラーハンドリングの徹底