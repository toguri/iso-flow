"""
NBA RSS Scraper DAG

このDAGは5分ごとにNBAのRSSフィードをスクレイピングし、
バックエンドAPIに送信します。
"""

from datetime import datetime, timedelta
import logging
import os
from typing import Dict, Any

from airflow import DAG
from airflow.operators.python import PythonOperator
from airflow.models import Variable
from airflow.providers.http.operators.http import SimpleHttpOperator
from airflow.providers.http.sensors.http import HttpSensor
from airflow.utils.dates import days_ago

# ロガーの設定
logger = logging.getLogger(__name__)

# デフォルト引数
default_args = {
    'owner': 'nba-tracker',
    'depends_on_past': False,
    'start_date': days_ago(1),
    'email_on_failure': False,
    'email_on_retry': False,
    'retries': 3,
    'retry_delay': timedelta(minutes=1),
}

# DAGの定義
dag = DAG(
    'nba_rss_scraper',
    default_args=default_args,
    description='NBA RSS feed scraper that runs every 5 minutes',
    schedule_interval='*/5 * * * *',  # 5分ごと
    catchup=False,
    max_active_runs=1,
    tags=['nba', 'scraping', 'rss'],
)

def get_backend_endpoint() -> str:
    """バックエンドAPIのエンドポイントを取得"""
    # Airflow Variableから取得（SSM Parameter Store経由）
    endpoint = Variable.get("backend_endpoint", default_var="/graphql")
    
    # 環境変数からも取得を試みる
    if not endpoint.startswith("http"):
        base_url = os.environ.get("BACKEND_API_URL", "http://localhost:8000")
        endpoint = f"{base_url}{endpoint}"
    
    return endpoint

def check_backend_health(**context) -> bool:
    """バックエンドAPIのヘルスチェック"""
    logger.info("Checking backend API health...")
    # HttpSensorの結果を使用
    return True

def trigger_scraping(**context) -> Dict[str, Any]:
    """スクレイピングをトリガーするGraphQL mutation"""
    endpoint = get_backend_endpoint()
    
    graphql_query = """
    mutation TriggerScraping {
        scrapeAndSaveRssFeeds {
            success
            message
            savedCount
            totalCount
            errors
        }
    }
    """
    
    logger.info(f"Triggering RSS scraping at {endpoint}")
    
    # SimpleHttpOperatorに渡すデータ
    return {
        'query': graphql_query,
        'variables': {}
    }

def process_scraping_result(**context) -> None:
    """スクレイピング結果を処理"""
    ti = context['task_instance']
    result = ti.xcom_pull(task_ids='trigger_rss_scraping')
    
    if result:
        logger.info(f"Scraping result: {result}")
        
        # 結果の解析
        if isinstance(result, dict) and 'data' in result:
            data = result['data'].get('scrapeAndSaveRssFeeds', {})
            
            if data.get('success'):
                logger.info(f"Successfully scraped {data.get('savedCount')} new items out of {data.get('totalCount')} total")
            else:
                logger.warning(f"Scraping completed with issues: {data.get('message')}")
                
            # エラーがある場合はログに記録
            errors = data.get('errors', [])
            if errors:
                for error in errors:
                    logger.error(f"Scraping error: {error}")
    else:
        logger.warning("No result received from scraping task")

# タスクの定義
# 1. バックエンドAPIのヘルスチェック
health_check = HttpSensor(
    task_id='check_backend_health',
    http_conn_id='backend_api',  # Airflow Connectionで設定
    endpoint='/health',
    poke_interval=30,
    timeout=300,
    mode='reschedule',  # リソースを節約
    dag=dag,
)

# 2. スクレイピングのトリガー
trigger_scraping_task = SimpleHttpOperator(
    task_id='trigger_rss_scraping',
    http_conn_id='backend_api',
    endpoint='/graphql',
    method='POST',
    headers={'Content-Type': 'application/json'},
    data='{{ ti.xcom_pull(task_ids="prepare_scraping_request") }}',
    xcom_push=True,
    dag=dag,
)

# 3. リクエストデータの準備
prepare_request = PythonOperator(
    task_id='prepare_scraping_request',
    python_callable=trigger_scraping,
    dag=dag,
)

# 4. 結果の処理
process_result = PythonOperator(
    task_id='process_scraping_result',
    python_callable=process_scraping_result,
    dag=dag,
)

# タスクの依存関係を設定
health_check >> prepare_request >> trigger_scraping_task >> process_result

# DAGのドキュメント
dag.doc_md = """
## NBA RSS Scraper DAG

このDAGは以下の処理を5分ごとに実行します：

1. **ヘルスチェック**: バックエンドAPIが稼働していることを確認
2. **スクレイピング実行**: GraphQL mutationを使用してRSSフィードをスクレイピング
3. **結果処理**: スクレイピング結果をログに記録し、必要に応じてアラートを送信

### 設定

以下の変数をAirflow VariablesまたはSSM Parameter Storeで設定してください：

- `backend_endpoint`: バックエンドAPIのGraphQLエンドポイント（例: http://alb-dns-name/graphql）

### 接続

以下のAirflow Connectionを設定してください：

- `backend_api`: バックエンドAPIへのHTTP接続
  - Host: ALBのDNS名またはIPアドレス
  - Schema: http または https
  - Port: 80 または 443

### モニタリング

- CloudWatch Logsでタスクログを確認できます
- 失敗時は3回までリトライされます
- 連続して失敗する場合は、バックエンドAPIの状態を確認してください
"""