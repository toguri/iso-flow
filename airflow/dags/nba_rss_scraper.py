"""
NBA RSS Scraper DAG

このDAGは5分ごとにNBAのRSSフィードをスクレイピングし、
データベースに保存します。
"""

from datetime import datetime, timedelta
import json
import logging

from airflow import DAG
from airflow.operators.python import PythonOperator
from airflow.providers.http.operators.http import SimpleHttpOperator
from airflow.providers.postgres.hooks.postgres import PostgresHook
from airflow.models import Variable
from airflow.utils.decorators import task

# ロガー設定
logger = logging.getLogger(__name__)

# デフォルト引数
default_args = {
    'owner': 'nba-trade-tracker',
    'depends_on_past': False,
    'start_date': datetime(2024, 7, 24),
    'email': ['alerts@nba-trade-tracker.com'],
    'email_on_failure': True,
    'email_on_retry': False,
    'retries': 3,
    'retry_delay': timedelta(minutes=1),
}

# DAG定義
dag = DAG(
    'nba_rss_scraper',
    default_args=default_args,
    description='NBA RSS Feed Scraper',
    schedule_interval='*/5 * * * *',  # 5分ごと
    catchup=False,
    tags=['production', 'scraping', 'nba'],
)

@task(dag=dag)
def call_scraper_api(**context):
    """
    バックエンドAPIのscrapeRss mutationを呼び出す
    """
    import requests
    
    # 環境変数からAPIエンドポイントを取得
    api_endpoint = Variable.get("BACKEND_API_ENDPOINT", "http://localhost:8000")
    
    # GraphQL mutation
    mutation = """
    mutation ScrapeRss {
        scrapeRss {
            savedCount
            skippedCount
            errorCount
            errors
        }
    }
    """
    
    try:
        response = requests.post(
            api_endpoint,
            json={"query": mutation},
            headers={"Content-Type": "application/json"},
            timeout=120  # 2分のタイムアウト
        )
        response.raise_for_status()
        
        result = response.json()
        
        if "errors" in result:
            logger.error(f"GraphQL errors: {result['errors']}")
            raise Exception(f"GraphQL errors: {result['errors']}")
        
        scrape_result = result["data"]["scrapeRss"]
        logger.info(
            f"Scraping completed: "
            f"{scrape_result['savedCount']} saved, "
            f"{scrape_result['skippedCount']} skipped, "
            f"{scrape_result['errorCount']} errors"
        )
        
        # 結果をXComに保存
        return scrape_result
        
    except requests.exceptions.RequestException as e:
        logger.error(f"API request failed: {str(e)}")
        raise
    except Exception as e:
        logger.error(f"Unexpected error: {str(e)}")
        raise

@task(dag=dag)
def validate_data_quality(**context):
    """
    データ品質をチェック
    """
    # 前のタスクの結果を取得
    scrape_result = context['task_instance'].xcom_pull(task_ids='call_scraper_api')
    
    # Aurora PostgreSQL接続
    pg_hook = PostgresHook(postgres_conn_id='aurora_postgres')
    
    # 品質チェック
    quality_checks = [
        {
            "check_name": "recent_data_exists",
            "sql": """
                SELECT COUNT(*) as count 
                FROM trade_news 
                WHERE scraped_at > NOW() - INTERVAL '10 minutes'
            """,
            "expected_min": 1
        },
        {
            "check_name": "no_null_categories",
            "sql": """
                SELECT COUNT(*) as count 
                FROM trade_news 
                WHERE category IS NULL 
                AND scraped_at > NOW() - INTERVAL '10 minutes'
            """,
            "expected_max": 0
        },
        {
            "check_name": "valid_categories",
            "sql": """
                SELECT COUNT(*) as count 
                FROM trade_news 
                WHERE category NOT IN ('Trade', 'Signing', 'Other')
                AND scraped_at > NOW() - INTERVAL '10 minutes'
            """,
            "expected_max": 0
        }
    ]
    
    failed_checks = []
    
    for check in quality_checks:
        result = pg_hook.get_first(check["sql"])
        count = result[0] if result else 0
        
        if "expected_min" in check and count < check["expected_min"]:
            failed_checks.append(
                f"{check['check_name']}: Expected at least {check['expected_min']}, got {count}"
            )
        
        if "expected_max" in check and count > check["expected_max"]:
            failed_checks.append(
                f"{check['check_name']}: Expected at most {check['expected_max']}, got {count}"
            )
    
    if failed_checks:
        logger.error(f"Data quality checks failed: {failed_checks}")
        raise ValueError(f"Data quality checks failed: {', '.join(failed_checks)}")
    
    logger.info("All data quality checks passed")
    
    # メトリクスを記録
    return {
        "quality_check_passed": True,
        "checks_performed": len(quality_checks),
        "scraping_stats": scrape_result
    }

@task(dag=dag)
def send_summary_notification(**context):
    """
    スクレイピング結果のサマリーを通知
    """
    # 前のタスクの結果を取得
    validation_result = context['task_instance'].xcom_pull(task_ids='validate_data_quality')
    
    # CloudWatch Metricsに送信（オプション）
    # ここではログ出力のみ
    logger.info(f"Daily scraping summary: {json.dumps(validation_result, indent=2)}")
    
    # Slackやメールへの通知もここで実装可能
    
    return "Notification sent"

# タスクの依存関係を定義
scrape_task = call_scraper_api()
validate_task = validate_data_quality()
notify_task = send_summary_notification()

scrape_task >> validate_task >> notify_task

# 追加のユーティリティDAG（日次集計など）
with DAG(
    'nba_daily_summary',
    default_args=default_args,
    description='NBA Daily Summary Generator',
    schedule_interval='0 9 * * *',  # 毎日午前9時（JST）
    catchup=False,
    tags=['production', 'reporting', 'nba'],
) as summary_dag:
    
    @task
    def generate_daily_report():
        """
        日次レポートを生成
        """
        pg_hook = PostgresHook(postgres_conn_id='aurora_postgres')
        
        # 過去24時間の統計を取得
        stats_sql = """
            SELECT 
                category,
                COUNT(*) as count,
                DATE(scraped_at) as date
            FROM trade_news
            WHERE scraped_at > NOW() - INTERVAL '24 hours'
            GROUP BY category, DATE(scraped_at)
            ORDER BY count DESC
        """
        
        results = pg_hook.get_records(stats_sql)
        
        report = {
            "date": datetime.now().strftime("%Y-%m-%d"),
            "categories": {},
            "total": 0
        }
        
        for category, count, date in results:
            report["categories"][category] = count
            report["total"] += count
        
        logger.info(f"Daily report generated: {json.dumps(report, indent=2)}")
        
        # S3に保存するなどの処理もここで実装可能
        
        return report
    
    daily_report_task = generate_daily_report()