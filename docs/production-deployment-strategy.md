# 本番環境デプロイ戦略

## 1. 現状評価と前提条件

### 既存リソース
- Terraformモジュール（VPC、RDS、ECS、S3、MWAA）
- CI/CDパイプライン（GitHub Actions）
- Dockerイメージ（バックエンド、フロントエンド）

### 必要な追加作業
1. **セキュリティ強化**
2. **本番環境設定**
3. **監視・運用体制**
4. **デプロイメント自動化**

## 2. デプロイ戦略フェーズ

### Phase 1: インフラストラクチャ準備（1-2日）
- [ ] 本番用Terraform環境の作成（`terraform/environments/prod/`）
- [ ] ネットワーク構成の確認（VPC、サブネット、セキュリティグループ）
- [ ] RDS Aurora Serverless v2のプロビジョニング
- [ ] ECS Fargateクラスターの準備
- [ ] S3 + CloudFrontの設定
- [ ] **Amazon MWAA（Managed Airflow）環境の構築**

### Phase 2: セキュリティ設定（1日）
- [ ] AWS Secrets Managerでの機密情報管理
- [ ] IAMロールとポリシーの最小権限設定
- [ ] WAFの設定（CloudFront）
- [ ] VPCセキュリティグループの厳格化
- [ ] データベース暗号化の確認
- [ ] **MWAA実行ロールの権限設定**

### Phase 3: CI/CD整備（1日）
- [ ] 本番デプロイ用GitHub Actionsワークフロー
- [ ] ブルーグリーンデプロイメントの実装
- [ ] 自動ロールバック機能
- [ ] デプロイ承認プロセス
- [ ] **Airflow DAGsの自動デプロイパイプライン**

### Phase 4: 監視・アラート設定（1日）
- [ ] CloudWatch Logsの設定
- [ ] CloudWatch Alarmsの設定
  - ECS タスクのCPU/メモリ使用率
  - RDS の接続数、CPU使用率
  - ALB のレスポンスタイム、エラー率
  - **MWAA タスクの実行状況、失敗率**
- [ ] AWS X-Rayでの分散トレーシング
- [ ] CloudWatch Dashboardの作成

### Phase 5: デプロイ実行（1-2日）
- [ ] データベースマイグレーション
- [ ] バックエンドサービスのデプロイ
- [ ] フロントエンドのデプロイ
- [ ] **Airflow DAGsのデプロイとスケジュール設定**
- [ ] DNSの設定（Route 53）
- [ ] SSL証明書の設定（ACM）

## 3. 本番環境固有の設定

### 環境変数（例）
```bash
# Backend
ENVIRONMENT=production
DATABASE_URL=<Aurora接続文字列>
AWS_REGION=ap-northeast-1
LOG_LEVEL=info
RUST_LOG=info

# Frontend
NEXT_PUBLIC_API_URL=https://api.example.com
NEXT_PUBLIC_ENVIRONMENT=production

# MWAA
AIRFLOW__CORE__PARALLELISM=16
AIRFLOW__CORE__DAG_CONCURRENCY=8
```

### インフラ設定の推奨値
```hcl
# RDS
min_capacity = 1.0
max_capacity = 4.0
backup_retention_period = 30
deletion_protection = true

# ECS
cpu = 1024
memory = 2048
desired_count = 2  # 高可用性のため

# CloudFront
price_class = "PriceClass_200"  # 日本を含むエッジロケーション

# MWAA
environment_class = "mw1.medium"
min_workers = 1
max_workers = 5
```

## 4. MWAA固有の考慮事項

### DAGsの管理
- [ ] S3バケットへのDAGファイルの配置
- [ ] requirements.txtの管理
- [ ] プラグインの配置
- [ ] 環境変数の設定

### スケジュール設定
```python
# 本番環境のDAGスケジュール例
nba_news_scraper = DAG(
    'nba_news_scraper',
    default_args=default_args,
    description='Scrape NBA news from multiple sources',
    schedule_interval='0 */6 * * *',  # 6時間ごと
    catchup=False,
)

data_cleanup = DAG(
    'data_cleanup',
    default_args=default_args,
    description='Clean up old data',
    schedule_interval='0 2 * * *',  # 毎日午前2時
    catchup=False,
)
```

### MWAAの監視項目
- DAG実行時間
- タスク失敗率
- ワーカー使用率
- スケジューラーの遅延

## 5. チェックリスト

### デプロイ前
- [ ] すべてのテストがパス
- [ ] セキュリティスキャンの完了
- [ ] パフォーマンステストの実施
- [ ] バックアップの確認
- [ ] ロールバック手順の文書化
- [ ] **Airflow DAGsのローカルテスト完了**

### デプロイ中
- [ ] データベースマイグレーションの成功確認
- [ ] ヘルスチェックの監視
- [ ] ログの監視
- [ ] エラー率の監視
- [ ] **MWAA環境の起動確認**

### デプロイ後
- [ ] エンドツーエンドテストの実行
- [ ] パフォーマンスメトリクスの確認
- [ ] セキュリティアラートの確認
- [ ] ユーザーアクセスの確認
- [ ] **Airflow DAGsの実行確認**

## 6. リスクと対策

### リスク1: データベース移行の失敗
- **対策**: 事前にステージング環境でテスト、バックアップの取得

### リスク2: トラフィック急増
- **対策**: Auto Scalingの設定、CloudFrontキャッシュの活用

### リスク3: セキュリティ侵害
- **対策**: WAF設定、定期的なセキュリティスキャン、最小権限の原則

### リスク4: コスト超過
- **対策**: AWS Budgetsの設定、定期的なコスト分析

### リスク5: MWAA DAGの失敗
- **対策**: リトライ設定、アラート通知、デッドレターキューの実装

## 7. 運用開始後のタスク

### 短期（1週間以内）
- [ ] 初期パフォーマンスチューニング
- [ ] ログ分析とアラート閾値の調整
- [ ] バックアップリストアのテスト
- [ ] **Airflowタスクの実行パターン分析**

### 中期（1ヶ月以内）
- [ ] 災害復旧計画の策定
- [ ] 運用手順書の作成
- [ ] インシデント対応プロセスの確立
- [ ] **DAGsの最適化とスケジュール調整**

### 長期（3ヶ月以内）
- [ ] コスト最適化の実施
- [ ] パフォーマンス最適化
- [ ] セキュリティ監査の実施
- [ ] **MWAAのスケーリング戦略の見直し**

## 8. 推奨される実行順序

1. **今すぐ**: Terraform本番環境設定の作成
2. **次に**: セキュリティ設定の実装（MWAA含む）
3. **その後**: CI/CDパイプラインの本番対応
4. **並行して**: 監視・アラートの設定
5. **最後に**: 段階的なデプロイ実行

## 9. 成功基準

- ゼロダウンタイムでのデプロイ
- 応答時間 < 200ms（p95）
- エラー率 < 0.1%
- 99.9%の可用性
- セキュリティアラート0件
- **Airflow DAGの成功率 > 99%**
- **データ収集の定期実行確認**