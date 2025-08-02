# Apache Airflow DAGs

NBA Trade TrackerのApache Airflow DAGsに関する詳細ドキュメントです。

## DAG一覧

### nba_rss_scraper

NBAのRSSフィードをスクレイピングして、トレード情報をデータベースに保存するDAGです。

#### 基本情報
- **スケジュール**: 5分ごと（`*/5 * * * *`）
- **目的**: NBAのRSSフィードをスクレイピングしてデータベースに保存
- **タスク構成**:
  1. バックエンドAPIのヘルスチェック
  2. GraphQL mutationでスクレイピング実行
  3. 結果の処理とログ記録

#### 実装詳細

```python
# DAGの主要パラメータ
default_args = {
    'owner': 'airflow',
    'depends_on_past': False,
    'start_date': datetime(2024, 1, 1),
    'email_on_failure': False,
    'email_on_retry': False,
    'retries': 1,
    'retry_delay': timedelta(minutes=1),
}
```

## セットアップ

### 1. 環境変数の設定

Terraform実行時に自動的に設定されますが、手動で設定する場合は以下をSSM Parameter Storeに登録：

```bash
# バックエンドAPIエンドポイント
aws ssm put-parameter \
  --name "/airflow/variables/backend_endpoint" \
  --value "http://your-alb-dns/graphql" \
  --type String

# スクレイピング間隔（分）
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

#### Terraformによる自動デプロイ

```bash
cd terraform/environments/dev
terraform apply -var="enable_mwaa=true"
```

#### 手動デプロイ

```bash
# DAGのアップロード
aws s3 cp dags/nba_rss_scraper.py s3://your-mwaa-bucket/dags/

# requirements.txtのアップロード
aws s3 cp requirements.txt s3://your-mwaa-bucket/
```

## モニタリング

### CloudWatch Logs

以下のロググループで各種ログを確認できます：

| ロググループ | 内容 |
|------------|------|
| `/aws/mwaa/environment-name/DAGProcessing` | DAGファイルの処理ログ |
| `/aws/mwaa/environment-name/Scheduler` | スケジューラーのログ |
| `/aws/mwaa/environment-name/Task` | タスク実行ログ |
| `/aws/mwaa/environment-name/WebServer` | Webサーバーのログ |
| `/aws/mwaa/environment-name/Worker` | ワーカーのログ |

### メトリクス

CloudWatch Metricsで以下を監視：

- DAG実行成功/失敗数
- タスク実行時間
- ワーカー使用率
- スケジューラーハートビート

## トラブルシューティング

### DAGが実行されない

1. **DAGが有効になっているか確認**
   - MWAA UIでDAGがPausedになっていないか確認
   
2. **スケジューラーログでエラーを確認**
   - CloudWatch Logs: `/aws/mwaa/environment-name/Scheduler`
   
3. **start_dateが過去の日付になっているか確認**
   - 未来の日付だとスケジュールされない

### バックエンドAPIに接続できない

1. **セキュリティグループの設定を確認**
   - MWAAからALBへのアクセスが許可されているか
   
2. **ALBのヘルスチェックステータスを確認**
   - ターゲットグループでヘルシーなインスタンスがあるか
   
3. **backend_api接続設定を確認**
   - Connection IDとホスト名が正しいか

### スクレイピングが失敗する

1. **タスクログで詳細なエラーを確認**
   - CloudWatch Logs: `/aws/mwaa/environment-name/Task`
   
2. **GraphQLエンドポイントの応答を確認**
   - GraphQL Playgroundで手動実行してみる
   
3. **データベース接続を確認**
   - RDSのステータスとセキュリティグループ

## 開発

### 新しいDAGの追加

1. `dags/`ディレクトリに新しいPythonファイルを作成
2. DAGの定義とタスクを実装
3. 必要な依存関係を`requirements.txt`に追加
4. S3にアップロードしてデプロイ

#### DAGテンプレート

```python
from datetime import datetime, timedelta
from airflow import DAG
from airflow.operators.python_operator import PythonOperator
from airflow.providers.http.operators.http import SimpleHttpOperator

default_args = {
    'owner': 'airflow',
    'depends_on_past': False,
    'start_date': datetime(2024, 1, 1),
    'email_on_failure': False,
    'retries': 1,
    'retry_delay': timedelta(minutes=5),
}

dag = DAG(
    'your_dag_name',
    default_args=default_args,
    description='DAGの説明',
    schedule_interval='0 */6 * * *',  # 6時間ごと
    catchup=False,
)

# タスクの定義
task1 = PythonOperator(
    task_id='your_task',
    python_callable=your_function,
    dag=dag,
)
```

### ローカルテスト

Apache Airflowをローカルにインストールしてテスト：

```bash
# 仮想環境の作成
python -m venv airflow-env
source airflow-env/bin/activate

# Airflowのインストール
pip install apache-airflow==2.8.1
pip install -r requirements.txt

# DAGの構文チェック
python dags/nba_rss_scraper.py

# Airflowスタンドアロンモードで実行
export AIRFLOW_HOME=$PWD
airflow db init
airflow standalone
```

## ベストプラクティス

1. **冪等性**: タスクは何度実行しても同じ結果になるように設計
2. **エラーハンドリング**: 適切なリトライとアラート設定
3. **リソース管理**: 不要なタスクは無効化してコスト削減
4. **ログ**: 重要な処理には適切なログを出力
5. **テスト**: DAGの変更は開発環境でテスト後にデプロイ

### コーディング規約

- PEP 8に準拠
- DAG IDは`snake_case`で記述
- タスクIDは動作を表す明確な名前を使用
- 十分なコメントとドキュメンテーション

### パフォーマンス最適化

- 並列実行可能なタスクは並列化
- 不要なデータ転送を避ける
- タスクの実行時間を監視して最適化