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
   - 詳細は [本番環境デプロイ戦略](./production-deployment-strategy.md) を参照
   - AWS環境（ECS、RDS、S3、CloudFront、MWAA）への段階的デプロイ

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

## 🚀 今後の機能拡張計画

### Feature Flag システムの導入

#### 背景
現在、翻訳サービスとしてAmazon Translateを実装しているが、翻訳品質の比較やコスト最適化のため、複数の翻訳サービスを動的に切り替えられる仕組みが必要。

#### 要件
- **リリースなしで翻訳プロバイダーを切り替え可能**
- **A/Bテストの実施**（例：50%のユーザーにOpenAI、50%にAmazon Translate）
- **段階的ロールアウト**（新しい翻訳サービスを徐々に展開）

#### 対応する翻訳サービス
1. Amazon Translate（実装済み）
2. OpenAI API (GPT-3.5/GPT-4)
3. Claude API (Anthropic)
4. DeepL API
5. Google Translate API
6. モックサービス（開発用、実装済み）

#### Feature Flag 管理サービスの選択肢

##### 1. **LaunchDarkly**
- **メリット**: 
  - 業界標準、豊富な機能
  - SDKが充実（Rust SDK有り）
  - リアルタイム更新
- **デメリット**: 
  - 有料（$75/月〜）
  - オーバースペックの可能性

##### 2. **AWS AppConfig**
- **メリット**: 
  - AWSエコシステムとの統合
  - 従量課金制
  - 既存のAWS環境で動作
- **デメリット**: 
  - UIが限定的
  - Rust SDKは自作が必要

##### 3. **Unleash**（オープンソース）
- **メリット**: 
  - 無料（セルフホスト）
  - カスタマイズ可能
  - Rust SDKあり
- **デメリット**: 
  - 運用コスト
  - インフラ管理が必要

##### 4. **シンプルな自前実装**
```rust
// 環境変数 + S3/DynamoDBでの設定管理
pub struct FeatureFlags {
    translation_provider: TranslationProvider,
    translation_weights: HashMap<TranslationProvider, f32>,
}
```

#### 実装計画

##### Phase 1: 基本実装（1-2週間）
- 環境変数ベースの切り替え機能
- 設定のホットリロード機能
- メトリクス収集（どのプロバイダーが使われたか）

##### Phase 2: Feature Flag サービス統合（2-3週間）
- Unleashのセルフホスト構築
- Rust SDKの統合
- 管理画面の設定

##### Phase 3: A/Bテスト機能（1-2週間）
- ユーザーセグメント機能
- 翻訳品質の比較メトリクス
- レポート機能

#### 技術的考慮事項

1. **キャッシュ戦略**
   - 同じテキストを複数のプロバイダーで翻訳する場合のキャッシュ
   - プロバイダー別のキャッシュキー設計

2. **フォールバック**
   - プライマリプロバイダーが失敗した場合の自動切り替え
   - レート制限への対応

3. **コスト管理**
   - プロバイダー別の使用量追跡
   - コスト上限の設定

4. **品質評価**
   - 翻訳品質の自動評価指標
   - ユーザーフィードバックの収集

#### 期待される効果
- 翻訳品質の継続的改善
- コストの最適化
- ベンダーロックインの回避
- 新しい翻訳サービスの低リスクな導入