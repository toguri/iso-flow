# CI/CD パフォーマンス最適化

## 概要

PR Checksワークフローのテスト実行時間を最適化し、開発者の待ち時間を短縮します。

## 主な改善点

### 1. cargo-nextestによる並列テスト実行
- 従来の`cargo test`から`cargo nextest`に移行
- テストの並列実行により実行時間を短縮
- 失敗したテストの自動リトライ機能

### 2. ジョブの統合と最適化
- `rust-checks`と`test`ジョブを`rust-all-checks`に統合
- 重複したPostgreSQLサービスの起動を削減
- セキュリティとドキュメントチェックをmatrix strategyで並列実行

### 3. キャッシュの改善
- restore-keysを追加して部分的なキャッシュヒットを有効化
- ツールのインストール状態をチェックして不要な再インストールを回避

### 4. テストグループごとの最適化
```toml
# データベーステスト - 並列度を制限
[[profile.ci.overrides]]
filter = "test(db::)"
test-threads = 2
retries = { backoff = "exponential", count = 3, delay = "2s" }

# 外部APIテスト - リトライ回数を増加
[[profile.ci.overrides]]
filter = "test(translation::)"
retries = { backoff = "exponential", count = 5, delay = "3s" }
```

## パフォーマンス比較（予測）

| ワークフロー | 従来 | 最適化後 | 改善率 |
|-------------|------|----------|--------|
| Rust Checks | 5分 | - | - |
| Test | 5分 | - | - |
| Coverage | 8分 | - | - |
| **統合後** | - | 8-10分 | 約40%短縮 |

※ 実際の改善率はテストの内容とGitHub Actionsのランナー性能により変動します

## 使用方法

### ローカルでnextestを使う場合
```bash
# インストール
cargo install cargo-nextest

# テスト実行
cargo nextest run

# プロファイルを指定して実行
cargo nextest run --profile ci
```

### 設定のカスタマイズ
`.config/nextest.toml`でテスト実行の詳細を調整できます：
- `test-threads`: 並列実行数
- `retries`: リトライ設定
- `slow-timeout`: タイムアウト設定

## 注意事項

1. **データベーステスト**: 並列度を上げすぎるとコネクション数の問題が発生する可能性があります
2. **外部APIテスト**: レート制限に注意が必要です
3. **初回実行**: nextestのインストールで追加時間がかかりますが、以降はキャッシュされます