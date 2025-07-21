# ISO Flow Documentation

このディレクトリはGitHub Pagesのルートディレクトリとして使用されます。

## 利用可能なドキュメント

- [Rust API Documentation](/rust-api/) - バックエンドAPIの詳細なドキュメント

## プロジェクト構成

```
iso-flow/
├── backend/        # Rustバックエンド
├── frontend/       # Next.jsフロントエンド（予定）
└── docs/           # ドキュメントルート
    └── index.html  # GitHub Pagesのエントリーポイント
```

## GitHub Pages設定手順

1. リポジトリの Settings > Pages にアクセス
2. Source を "Deploy from a branch" に設定
3. Branch を "main" / "docs" に設定
4. Save をクリック

数分後、`https://toguri.github.io/iso-flow/` でドキュメントが公開されます。