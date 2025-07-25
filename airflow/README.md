# Airflow DAGs for NBA Trade Tracker

このディレクトリには、NBA Trade TrackerのApache Airflow DAGが含まれています。

## 概要

Amazon MWAA (Managed Workflows for Apache Airflow) を使用して、定期的なスクレイピングタスクを実行します。

## DAG一覧

### nba_rss_scraper
- **スケジュール**: 5分ごと（`*/5 * * * *`）
- **目的**: NBAのRSSフィードをスクレイピングしてデータベースに保存
- **タスク**:
  1. バックエンドAPIのヘルスチェック
  2. GraphQL mutationでスクレイピング実行
  3. 結果の処理とログ記録

## セットアップ

### 1. 環境変数の設定

Terraform実行時に自動的に設定されますが、手動で設定する場合は以下をSSM Parameter Storeに登録：

```bash
aws ssm put-parameter \
  --name "/airflow/variables/backend_endpoint" \
  --value "http://your-alb-dns/graphql" \
  --type String

aws ssm put-parameter \
  --name "/airflow/variables/scraping_interval_minutes" \
  --value "5" \
  --type String
```

### 2. Airflow接続の設定

MWAA Webserver UIから、以下の接続を設定：

**Connection ID**: `backend_api`
- **Connection Type**: HTTP
- **Host**: ALBのDNS名
- **Schema**: http
- **Port**: 80

### 3. DAGのデプロイ

Terraformを実行すると自動的にS3にアップロードされます：

```bash
cd terraform/environments/dev
terraform apply -var="enable_mwaa=true"
```

手動でアップロードする場合：

```bash
# DAGのアップロード
aws s3 cp dags/nba_rss_scraper.py s3://your-mwaa-bucket/dags/

# requirements.txtのアップロード
aws s3 cp requirements.txt s3://your-mwaa-bucket/
```

## モニタリング

### CloudWatch Logs

以下のロググループで各種ログを確認できます：

- `/aws/mwaa/environment-name/DAGProcessing`
- `/aws/mwaa/environment-name/Scheduler`
- `/aws/mwaa/environment-name/Task`
- `/aws/mwaa/environment-name/WebServer`
- `/aws/mwaa/environment-name/Worker`

### メトリクス

CloudWatch Metricsで以下を監視：

- DAG実行成功/失敗数
- タスク実行時間
- ワーカー使用率
- スケジューラーハートビート

## トラブルシューティング

### DAGが実行されない

1. DAGが有効になっているか確認
2. スケジューラーログでエラーを確認
3. start_dateが過去の日付になっているか確認

### バックエンドAPIに接続できない

1. セキュリティグループの設定を確認
2. ALBのヘルスチェックステータスを確認
3. backend_api接続設定を確認

### スクレイピングが失敗する

1. タスクログで詳細なエラーを確認
2. GraphQLエンドポイントの応答を確認
3. データベース接続を確認

## 開発

### 新しいDAGの追加

1. `dags/`ディレクトリに新しいPythonファイルを作成
2. DAGの定義とタスクを実装
3. 必要な依存関係を`requirements.txt`に追加
4. S3にアップロードしてデプロイ

### ローカルテスト

Apache Airflowをローカルにインストールしてテスト：

```bash
pip install apache-airflow==2.8.1
pip install -r requirements.txt

# DAGの構文チェック
python dags/nba_rss_scraper.py

# Airflowスタンドアロンモードで実行
airflow standalone
```

## ベストプラクティス

1. **冪等性**: タスクは何度実行しても同じ結果になるように
2. **エラーハンドリング**: 適切なリトライとアラート設定
3. **リソース管理**: 不要なタスクは無効化してコスト削減
4. **ログ**: 重要な処理には適切なログを出力
5. **テスト**: DAGの変更は開発環境でテスト後にデプロイ