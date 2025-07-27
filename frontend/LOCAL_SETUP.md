# フロントエンド ローカル環境セットアップ

## 前提条件

- Java 11以上
- Node.js 16以上（Gradleが自動的にインストール）
- バックエンドが起動していること（http://localhost:8000）

## 起動方法

### 1. 開発サーバーの起動

```bash
cd frontend
./gradlew jsBrowserDevelopmentRun --continuous
```

これにより：
- http://localhost:8080 でフロントエンドが起動
- ファイル変更時に自動的にリロード
- ホットリロード対応

### 2. プロダクションビルド

```bash
./gradlew jsBrowserProductionWebpack
```

ビルド結果は `build/dist/js/productionExecutable/` に出力されます。

### 3. テストの実行

```bash
# 単体テスト
./gradlew jsTest

# ブラウザテスト
./gradlew jsBrowserTest
```

## 設定

### GraphQL エンドポイント

`src/jsMain/kotlin/api/GraphQLClient.kt`:
```kotlin
private val endpoint = "http://localhost:8000/graphql"
```

### カテゴリ

- Trade（トレード）
- Signing（契約）
- Other（その他）

## トラブルシューティング

### CORS エラーが発生する場合

バックエンドが正しくCORSを設定しているか確認：
```bash
curl -I http://localhost:8000/graphql \
  -H "Origin: http://localhost:8080"
```

### GraphQL クエリが失敗する場合

1. バックエンドが起動しているか確認
2. GraphQL Playground で直接クエリを実行：http://localhost:8000
3. ブラウザの開発者ツールでネットワークタブを確認

### ビルドエラー

```bash
# クリーンビルド
./gradlew clean build
```

## 開発のヒント

1. **Compose for Web**
   - `@Composable` 関数でUIを構築
   - `remember` と `mutableStateOf` で状態管理

2. **スタイリング**
   - CSS-in-JSスタイル
   - `style {}` ブロック内でCSSプロパティを定義

3. **非同期処理**
   - Kotlin Coroutines使用
   - `LaunchedEffect` でコンポーネントのライフサイクル管理