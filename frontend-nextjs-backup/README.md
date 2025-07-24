# NBA Trade Tracker Frontend

NBA Trade TrackerのNext.jsフロントエンドアプリケーション。

## 技術スタック

- Next.js 14 (App Router)
- TypeScript
- Tailwind CSS
- Apollo Client (GraphQL)

## セットアップ

```bash
# 依存関係のインストール
npm install

# 開発サーバーの起動
npm run dev
```

## 環境変数

`.env.local`ファイルを作成：

```
NEXT_PUBLIC_GRAPHQL_URL=http://localhost:8000/graphql
```

## 開発

1. バックエンドサーバーを起動
```bash
cd ../backend
cargo run
```

2. フロントエンドを起動
```bash
npm run dev
```

3. http://localhost:3000 にアクセス

## 機能

- ニュース一覧表示
- カテゴリーフィルター（Trade/Signing/Other）
- リアルタイム更新ボタン
- レスポンシブデザイン