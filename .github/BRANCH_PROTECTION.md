# Branch Protection Rules Update Guide

このドキュメントは、統合CI導入に伴うブランチ保護ルールの更新手順を説明します。

## 背景

従来の問題：
- リバートPRで必要なチェックが実行されない
- コード変更がないPRでもフルテストが要求される
- 8個の必須チェックのうち一部しか実行されない

解決策：
- 統合CIワークフロー（unified-ci.yml）の導入
- PRタイプに応じた適切なチェックの実行
- 単一の必須ステータスチェック

## 新しいブランチ保護設定

### 1. GitHub Settings → Branches → mainルールを編集

### 2. 必須ステータスチェックの更新

**削除する必須チェック：**
- ❌ Test (Rust CI)
- ❌ Security Audit (Rust CI)
- ❌ Generate Coverage Report (Code Coverage)
- ❌ Build Documentation (Documentation)
- ❌ Check Commit Messages (PR Check)
- ❌ Auto Label PR (PR Check)
- ❌ codecov/patch
- ❌ codecov/project

**追加する必須チェック：**
- ✅ Final Status Check (Unified CI)

### 3. 設定内容

```
✅ Require a pull request before merging
  ✅ Require approvals: 1
  ✅ Dismiss stale pull request approvals when new commits are pushed
  
✅ Require status checks to pass before merging
  必須チェック:
  - Final Status Check
  
  ✅ Require branches to be up to date before merging
  
✅ Require conversation resolution before merging

✅ Do not allow bypassing the above settings
```

## PRタイプ別の動作

### 1. リバートPR（revert:, chore: revert）
- 共通チェック（コミットメッセージ、PRフォーマット）のみ
- コードテスト、ビルド、カバレッジはスキップ

### 2. ドキュメントPR（docs:）
- 共通チェック + ドキュメントビルド
- コードテストはスキップ

### 3. CI/CD PR（ci:, chore: ci）
- 共通チェック + 基本的なテスト
- カバレッジはスキップ

### 4. コード変更PR（feat:, fix:, refactor:, test:）
- フルチェック（テスト、セキュリティ監査、カバレッジ）

### 5. その他のPR
- ファイル変更内容に基づいて自動判定

## 移行手順

1. このPRをマージ
2. GitHub Settingsでブランチ保護ルールを更新
3. 既存のPRは自動的に新しいチェックで再実行される

## メリット

- **効率的**: 不要なチェックを実行しない
- **柔軟**: PRタイプに応じた適切な検証
- **シンプル**: 単一の必須チェックで管理
- **明確**: PRタイプが自動的に表示される

## トラブルシューティング

### Q: 新しいチェックが表示されない
A: PRを一度クローズして再オープンするか、空コミットをプッシュしてください。

### Q: 古いチェックがまだ実行される
A: 問題ありません。古いチェックは必須ではないため、無視できます。

### Q: Final Status Checkが失敗する
A: ログを確認して、どのステップで失敗したか確認してください。PRタイプに応じて必要なチェックのみが実行されます。