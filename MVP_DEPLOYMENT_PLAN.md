# MVP外部公開計画 - エンタープライズグレードアーキテクチャ

最終更新: 2024年7月23日

## 🎯 目標
**技術修練を兼ねて、本格的なクラウドネイティブアーキテクチャで外部公開を実現する**

## 📊 現在の状況（2024年7月23日時点）

### ✅ 実装済み機能
- **バックエンド**
  - ESPN RSS取得・解析機能
  - GraphQL API（ニュース一覧、カテゴリー別取得）
  - カテゴリー自動判定（Trade/Signing/Other）
  - CORS対応（ポート8000）
  
- **フロントエンド**
  - Kotlin Compose for Webによる実装
  - ニュース一覧表示（カード形式）
  - カテゴリーフィルター（すべて/Trade/Signing/その他）
  - 日本時間での日付表示

- **インフラ・CI/CD**
  - GitHub Actions（自動テスト、ビルド、品質チェック）
  - ブランチ保護ルール

## 🏗️ 提案アーキテクチャ

### システム構成図
```
┌─────────────────────────────────────────────────────────────────┐
│                          CloudFront                              │
│                     (CDN + カスタムドメイン)                      │
└────────────────────────────┬────────────────────────────────────┘
                             │
        ┌────────────────────┴────────────────────┐
        │                                         │
┌───────▼────────┐                    ┌──────────▼──────────┐
│   S3 Bucket    │                    │   ALB + ECS Fargate │
│  (Frontend)    │                    │    (Backend API)    │
│ Kotlin Compose │                    │   Rust GraphQL      │
└────────────────┘                    └──────────┬──────────┘
                                                 │
                                      ┌──────────▼──────────┐
                                      │    RDS Aurora       │
                                      │   (PostgreSQL)      │
                                      └──────────▲──────────┘
                                                 │
┌─────────────────────────────────────────────────┴───────────┐
│                    Amazon MWAA (Managed Airflow)            │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐   │
│  │ RSS Scraper │  │ Data Quality │  │ Notification    │   │
│  │    DAG      │  │     DAG      │  │      DAG        │   │
│  └─────────────┘  └──────────────┘  └─────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                                │
                    ┌───────────▼────────────┐
                    │    CloudWatch Logs     │
                    │   Metrics & Alarms     │
                    └────────────────────────┘
```

### 技術スタック詳細

#### 1. **フロントエンド**
- **ホスティング**: S3 + CloudFront
- **CI/CD**: GitHub Actions → S3デプロイ
- **監視**: CloudWatch RUM (Real User Monitoring)

#### 2. **バックエンド**
- **コンテナ**: ECS Fargate (オートスケーリング対応)
- **ロードバランサー**: Application Load Balancer
- **API Gateway**: 将来的にAppSyncも検討
- **CI/CD**: GitHub Actions → ECR → ECS

#### 3. **データベース**
- **メイン**: Aurora PostgreSQL Serverless v2
- **キャッシュ**: ElastiCache for Redis
- **バックアップ**: 自動スナップショット + S3エクスポート

#### 4. **ワークフロー管理**
- **Amazon MWAA (Managed Apache Airflow)**
  - RSSスクレイピングDAG（5分間隔）
  - データ品質チェックDAG（1時間間隔）
  - 通知・アラートDAG
  - S3へのデータエクスポートDAG

#### 5. **監視・ログ**
- **CloudWatch**: ログ集約、メトリクス、アラーム
- **X-Ray**: 分散トレーシング
- **SNS**: アラート通知

## 📅 実装スケジュール（2週間）

### Week 1: インフラ構築とデータ永続化

#### Day 1-2: AWS基盤構築
- [ ] AWS Organizations + Control Tower セットアップ
- [ ] VPC設計・構築（Public/Private Subnet）
- [ ] IAMロール・ポリシー設計
- [ ] Terraform/CDKプロジェクト初期化

#### Day 3-4: データベース層
- [ ] Aurora PostgreSQL Serverless v2構築
- [ ] SQLxマイグレーション適用
- [ ] バックエンドのDB接続実装
- [ ] ElastiCache構築（オプション）

#### Day 5-7: MWAA環境構築
- [ ] MWAA環境作成
- [ ] S3バケット（DAGs用）設定
- [ ] RSSスクレイピングDAG実装
  ```python
  from airflow import DAG
  from airflow.providers.http.operators.http import SimpleHttpOperator
  from airflow.providers.postgres.operators.postgres import PostgresOperator
  
  with DAG('nba_rss_scraper', 
           schedule_interval='*/5 * * * *',
           catchup=False) as dag:
      
      scrape_rss = SimpleHttpOperator(
          task_id='scrape_espn_rss',
          endpoint='scraper/execute',
          method='POST'
      )
      
      check_quality = PostgresOperator(
          task_id='check_data_quality',
          sql='SELECT COUNT(*) FROM news_items WHERE created_at > NOW() - INTERVAL 5 MINUTE'
      )
  ```

### Week 2: アプリケーションデプロイと最適化

#### Day 8-9: コンテナ化とECS構築
- [ ] Dockerfile最適化（マルチステージビルド）
- [ ] ECRリポジトリ作成
- [ ] ECS Fargateクラスター構築
- [ ] ALB + Auto Scaling設定

#### Day 10-11: フロントエンドデプロイ
- [ ] S3バケット作成（静的ホスティング）
- [ ] CloudFrontディストリビューション設定
- [ ] Route 53（カスタムドメイン）
- [ ] GitHub Actions CD設定

#### Day 12-13: 監視・セキュリティ
- [ ] CloudWatch Dashboards作成
- [ ] X-Ray統合
- [ ] WAF設定（SQL Injection, XSS対策）
- [ ] Secrets Manager統合

#### Day 14: 総合テスト
- [ ] 負荷テスト（Locust/K6）
- [ ] セキュリティスキャン
- [ ] 災害復旧テスト
- [ ] ドキュメント完成

## 💰 コスト見積もり（月額）

| サービス | 構成 | 推定コスト |
|---------|------|-----------|
| MWAA | Small環境 | $320 |
| Aurora Serverless v2 | 0.5 ACU最小 | $50 |
| ECS Fargate | 0.25 vCPU, 0.5GB | $20 |
| ALB | 基本料金 + LCU | $25 |
| S3 + CloudFront | 10GB転送 | $5 |
| その他（CloudWatch等） | - | $20 |
| **合計** | | **$440/月** |

※ 無料枠やReserved Instancesで削減可能

## 🚀 技術的チャレンジと学習ポイント

### 1. **Infrastructure as Code**
- Terraform or AWS CDKでの完全自動化
- GitOpsワークフロー実装

### 2. **Airflow DAG開発**
- カスタムオペレーター作成
- 動的DAG生成
- エラーハンドリングとリトライ戦略

### 3. **オブザーバビリティ**
- 分散トレーシング実装
- カスタムメトリクス設計
- SLI/SLO定義

### 4. **セキュリティ**
- ゼロトラストネットワーク
- SAST/DAST統合
- コンプライアンス対応

## 📝 Airflow DAG実装例

### RSS スクレイピング DAG
```python
from datetime import datetime, timedelta
from airflow import DAG
from airflow.providers.http.hooks.http import HttpHook
from airflow.providers.postgres.hooks.postgres import PostgresHook
from airflow.operators.python import PythonOperator
from airflow.models import Variable

default_args = {
    'owner': 'nba-tracker',
    'depends_on_past': False,
    'start_date': datetime(2024, 7, 23),
    'email_on_failure': True,
    'email_on_retry': False,
    'retries': 3,
    'retry_delay': timedelta(minutes=1),
}

def scrape_and_store(**context):
    # Backend APIを呼び出し
    http_hook = HttpHook(http_conn_id='backend_api', method='POST')
    response = http_hook.run('/graphql', 
                           json={"query": "mutation { scrapeRss }"})
    
    # 結果をXComに保存
    context['task_instance'].xcom_push(
        key='scraped_count', 
        value=response.json()['data']['scrapeRss']['count']
    )

def validate_data(**context):
    pg_hook = PostgresHook(postgres_conn_id='aurora_db')
    
    # 最新データの品質チェック
    quality_checks = [
        "SELECT COUNT(*) FROM news_items WHERE created_at > NOW() - INTERVAL '5 minutes'",
        "SELECT COUNT(*) FROM news_items WHERE category IS NULL AND created_at > NOW() - INTERVAL '5 minutes'"
    ]
    
    for check in quality_checks:
        result = pg_hook.get_first(check)
        if result[0] == 0:
            raise ValueError(f"Data quality check failed: {check}")

with DAG('nba_rss_scraper',
         default_args=default_args,
         description='NBA RSS Feed Scraper',
         schedule_interval='*/5 * * * *',
         catchup=False,
         tags=['production', 'scraping']) as dag:
    
    scrape_task = PythonOperator(
        task_id='scrape_rss_feeds',
        python_callable=scrape_and_store,
    )
    
    validate_task = PythonOperator(
        task_id='validate_data_quality',
        python_callable=validate_data,
    )
    
    scrape_task >> validate_task
```

## 🎯 成功指標とSLO

### Service Level Objectives (SLO)
- **可用性**: 99.9%（月間ダウンタイム < 44分）
- **レイテンシ**: P95 < 200ms
- **エラー率**: < 0.1%
- **データ鮮度**: 最新RSSから5分以内

### Key Performance Indicators (KPI)
- デプロイ頻度: 週3回以上
- MTTR: < 30分
- インフラコスト: 予算内（$500/月）
- セキュリティスコア: A評価

## 🔧 運用手順書

### 1. 日次運用
- CloudWatch Dashboardチェック
- Airflow DAG実行状況確認
- コスト監視

### 2. 週次運用
- セキュリティパッチ適用
- パフォーマンスレビュー
- バックアップ検証

### 3. 月次運用
- コスト最適化レビュー
- キャパシティプランニング
- 災害復旧訓練

---

このアーキテクチャは、NBA Trade Trackerを単なるMVPではなく、エンタープライズグレードのプロダクションシステムとして構築します。技術修練として、AWS Well-Architected Frameworkに準拠した本格的なクラウドネイティブアプリケーションを実現します。