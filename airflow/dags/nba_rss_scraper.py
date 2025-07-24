"""
NBA RSS Scraper DAG

This DAG scrapes NBA trade news from RSS feeds every 5 minutes.
It calls the backend GraphQL API to trigger the scraping process.
"""

from datetime import datetime, timedelta
import requests
import json
from airflow import DAG
from airflow.operators.python_operator import PythonOperator
from airflow.models import Variable

# Default arguments for the DAG
default_args = {
    'owner': 'nba-trade-tracker',
    'depends_on_past': False,
    'start_date': datetime(2025, 7, 24),
    'email_on_failure': False,
    'email_on_retry': False,
    'retries': 1,
    'retry_delay': timedelta(minutes=1),
}

# Define the DAG
dag = DAG(
    'nba_rss_scraper',
    default_args=default_args,
    description='Scrape NBA trade news from RSS feeds',
    schedule_interval=timedelta(minutes=5),
    catchup=False,
    tags=['nba', 'scraping', 'rss'],
)

def scrape_rss_feeds(**context):
    """
    Call the backend GraphQL API to trigger RSS scraping
    """
    # Get the backend API endpoint from Airflow Variables
    # This should be set in the MWAA environment
    backend_url = Variable.get("BACKEND_API_ENDPOINT", default_var="http://localhost:8000")
    
    # GraphQL mutation to trigger scraping
    mutation = """
    mutation {
        scrapeRss {
            success
            message
            itemsProcessed
        }
    }
    """
    
    # Prepare the request
    headers = {
        'Content-Type': 'application/json',
    }
    
    payload = {
        'query': mutation
    }
    
    try:
        # Make the request
        response = requests.post(
            backend_url,
            headers=headers,
            data=json.dumps(payload),
            timeout=30
        )
        
        # Check response
        response.raise_for_status()
        
        # Parse response
        data = response.json()
        
        if 'errors' in data:
            raise Exception(f"GraphQL errors: {data['errors']}")
        
        # Log success
        result = data.get('data', {}).get('scrapeRss', {})
        print(f"Scraping completed: {result}")
        
        # Push result to XCom for downstream tasks
        context['task_instance'].xcom_push(key='scrape_result', value=result)
        
        return result
        
    except requests.exceptions.RequestException as e:
        print(f"Request failed: {str(e)}")
        raise
    except Exception as e:
        print(f"Scraping failed: {str(e)}")
        raise

def check_scraping_result(**context):
    """
    Check the result of the scraping operation
    """
    # Get result from previous task
    result = context['task_instance'].xcom_pull(
        task_ids='scrape_rss_feeds',
        key='scrape_result'
    )
    
    if not result:
        raise Exception("No scraping result found")
    
    # Check if scraping was successful
    if not result.get('success', False):
        raise Exception(f"Scraping failed: {result.get('message', 'Unknown error')}")
    
    items_processed = result.get('itemsProcessed', 0)
    print(f"Successfully processed {items_processed} news items")
    
    # You can add additional checks or notifications here
    # For example, send a notification if no new items were found
    if items_processed == 0:
        print("Warning: No new items were found in this scraping run")

# Define the tasks
scrape_task = PythonOperator(
    task_id='scrape_rss_feeds',
    python_callable=scrape_rss_feeds,
    dag=dag,
)

check_task = PythonOperator(
    task_id='check_scraping_result',
    python_callable=check_scraping_result,
    dag=dag,
)

# Set task dependencies
scrape_task >> check_task