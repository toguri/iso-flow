# NBA Trade Tracker Frontend

NBA Trade TrackerのKotlin/JS Compose for Webフロントエンドアプリケーション。

## 技術スタック

- Kotlin/JS
- Compose for Web
- Apollo GraphQL Client for Kotlin
- Kotlinx Serialization
- Gradle

## セットアップ

### 前提条件
- JDK 11以上
- Gradle 7.6以上

### 依存関係のインストールとビルド
```bash
./gradlew build
```

### 開発サーバーの起動
```bash
./gradlew jsBrowserDevelopmentRun --continuous
```

開発サーバーが起動すると、自動的にブラウザが開きます。
コードを変更すると自動的に再ビルドされ、ブラウザがリロードされます。

## プロジェクト構成

```
frontend/
├── build.gradle.kts        # ビルド設定
├── settings.gradle.kts     # プロジェクト設定
├── gradle.properties       # Gradle設定
└── src/
    └── jsMain/
        ├── kotlin/
        │   ├── Main.kt     # エントリーポイント
        │   ├── App.kt      # メインアプリケーション
        │   ├── api/        # GraphQL クライアント
        │   ├── components/ # UIコンポーネント
        │   └── models/     # データモデル
        └── resources/
            └── index.html  # HTMLテンプレート
```

## 開発

1. バックエンドサーバーを起動
```bash
cd ../backend
cargo run --bin nba-trade-scraper
```

2. フロントエンドを起動
```bash
./gradlew jsBrowserDevelopmentRun --continuous
```

3. http://localhost:8080 にアクセス（自動的に開きます）

## 機能

- ニュース一覧表示
- カテゴリーフィルター（All/Trade/Signing/Other）
- リアルタイム更新ボタン
- レスポンシブデザイン

## ビルド

### プロダクションビルド
```bash
./gradlew jsBrowserProductionWebpack
```

ビルド成果物は `build/distributions/` に生成されます。