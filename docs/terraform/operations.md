# 運用ガイド

NBA Trade Trackerインフラストラクチャの運用に関するガイドです。

## 監視とアラート

### CloudWatch ダッシュボード

#### カスタムダッシュボードの作成

```bash
# CloudWatch ダッシュボードの作成
aws cloudwatch put-dashboard \
  --dashboard-name "NBATracker-Dev" \
  --dashboard-body file://dashboard.json
```

#### 主要メトリクス

1. **ECS メトリクス**
   - CPUUtilization
   - MemoryUtilization
   - RunningTaskCount
   - HealthyTargetCount

2. **RDS メトリクス**
   - DatabaseConnections
   - CPUUtilization  
   - FreeableMemory
   - ReadLatency/WriteLatency

3. **ALB メトリクス**
   - RequestCount
   - TargetResponseTime
   - HTTPCode_Target_4XX_Count
   - HTTPCode_Target_5XX_Count

4. **MWAA メトリクス**
   - SchedulerHeartbeat
   - DAGProcessingSuccess
   - TaskInstanceSuccess/Failed
   - WorkerCount

### アラート設定

#### SNS トピックの作成

```bash
# SNS トピックの作成
aws sns create-topic --name nba-tracker-alerts

# Email サブスクリプションの追加
aws sns subscribe \
  --topic-arn arn:aws:sns:ap-northeast-1:123456789012:nba-tracker-alerts \
  --protocol email \
  --notification-endpoint your-email@example.com
```

#### CloudWatch アラームの設定例

```bash
# ECS CPU 高使用率アラーム
aws cloudwatch put-metric-alarm \
  --alarm-name "nba-tracker-ecs-cpu-high" \
  --alarm-description "ECS CPU utilization is too high" \
  --metric-name CPUUtilization \
  --namespace AWS/ECS \
  --statistic Average \
  --period 300 \
  --threshold 80 \
  --comparison-operator GreaterThanThreshold \
  --evaluation-periods 2 \
  --alarm-actions arn:aws:sns:ap-northeast-1:123456789012:nba-tracker-alerts
```

## ログ管理

### ログの確認

#### ECS タスクログ

```bash
# 最新のログを表示
aws logs tail /aws/ecs/nba-tracker-dev --follow

# 特定の時間範囲のログを検索
aws logs filter-log-events \
  --log-group-name /aws/ecs/nba-tracker-dev \
  --start-time $(date -u -d '1 hour ago' +%s)000 \
  --filter-pattern "ERROR"
```

#### MWAA ログ

```bash
# DAG 実行ログ
aws logs tail /aws/mwaa/nba-tracker-dev/task --follow

# スケジューラーログ
aws logs tail /aws/mwaa/nba-tracker-dev/scheduler --follow
```

### ログの分析

#### CloudWatch Insights クエリ例

```sql
-- エラーログの集計
fields @timestamp, @message
| filter @message like /ERROR/
| stats count() by bin(5m)

-- レスポンスタイムの分析
fields @timestamp, duration
| filter @message like /Request completed/
| stats avg(duration), max(duration), min(duration) by bin(5m)
```

## バックアップとリストア

### データベースバックアップ

#### 手動スナップショットの作成

```bash
# スナップショットの作成
aws rds create-db-cluster-snapshot \
  --db-cluster-snapshot-identifier nba-tracker-manual-$(date +%Y%m%d%H%M%S) \
  --db-cluster-identifier nba-tracker-dev

# スナップショットの一覧
aws rds describe-db-cluster-snapshots \
  --db-cluster-identifier nba-tracker-dev
```

#### リストア手順

```bash
# 新しいクラスターとしてリストア
aws rds restore-db-cluster-from-snapshot \
  --db-cluster-identifier nba-tracker-dev-restored \
  --snapshot-identifier nba-tracker-manual-20240101120000 \
  --engine aurora-postgresql
```

### アプリケーションバックアップ

#### 設定のエクスポート

```bash
# Secrets Manager のバックアップ
aws secretsmanager get-secret-value \
  --secret-id nba-tracker-dev-db-password \
  > secrets-backup.json

# Parameter Store のバックアップ
aws ssm get-parameters-by-path \
  --path /nba-tracker/dev \
  --recursive \
  > parameters-backup.json
```

## スケーリング

### ECS のスケーリング

#### 手動スケーリング

```bash
# サービスのタスク数を変更
aws ecs update-service \
  --cluster nba-tracker-dev \
  --service nba-tracker-dev \
  --desired-count 3
```

#### Auto Scaling の設定

```bash
# スケーリングターゲットの登録
aws application-autoscaling register-scalable-target \
  --service-namespace ecs \
  --resource-id service/nba-tracker-dev/nba-tracker-dev \
  --scalable-dimension ecs:service:DesiredCount \
  --min-capacity 1 \
  --max-capacity 10

# スケーリングポリシーの作成
aws application-autoscaling put-scaling-policy \
  --service-namespace ecs \
  --scalable-dimension ecs:service:DesiredCount \
  --resource-id service/nba-tracker-dev/nba-tracker-dev \
  --policy-name cpu-scaling-policy \
  --policy-type TargetTrackingScaling \
  --target-tracking-scaling-policy-configuration '{
    "TargetValue": 70.0,
    "PredefinedMetricSpecification": {
      "PredefinedMetricType": "ECSServiceAverageCPUUtilization"
    }
  }'
```

### RDS のスケーリング

```bash
# Aurora Serverless v2 の ACU 調整
aws rds modify-db-cluster \
  --db-cluster-identifier nba-tracker-dev \
  --serverless-v2-scaling-configuration '{
    "MinCapacity": 0.5,
    "MaxCapacity": 2.0
  }' \
  --apply-immediately
```

## トラブルシューティング

### 一般的な問題と解決方法

#### 1. ECS タスクが起動しない

**確認手順：**

```bash
# タスクの状態を確認
aws ecs describe-tasks \
  --cluster nba-tracker-dev \
  --tasks $(aws ecs list-tasks --cluster nba-tracker-dev --query 'taskArns[0]' --output text)

# 停止理由を確認
aws ecs describe-tasks \
  --cluster nba-tracker-dev \
  --tasks $(aws ecs list-tasks --cluster nba-tracker-dev --desired-status STOPPED --query 'taskArns[0]' --output text)
```

**一般的な原因：**
- ECR イメージが存在しない
- タスクロールの権限不足
- メモリ/CPU の不足
- ヘルスチェックの失敗

#### 2. データベース接続エラー

**確認手順：**

```bash
# RDS の状態確認
aws rds describe-db-clusters \
  --db-cluster-identifier nba-tracker-dev

# セキュリティグループの確認
aws ec2 describe-security-groups \
  --group-ids sg-xxxxxxxxx
```

**一般的な原因：**
- セキュリティグループの設定ミス
- データベースが一時停止中
- 認証情報の不一致

#### 3. MWAA DAG が実行されない

**確認手順：**

```bash
# MWAA 環境の状態確認
aws mwaa get-environment --name nba-tracker-dev

# S3 の DAG ファイル確認
aws s3 ls s3://nba-tracker-dev-mwaa/dags/
```

**一般的な原因：**
- DAG ファイルの構文エラー
- スケジュール設定の問題
- 依存関係の不足

### 緊急時の対応

#### サービスの緊急停止

```bash
# ECS サービスの停止
aws ecs update-service \
  --cluster nba-tracker-dev \
  --service nba-tracker-dev \
  --desired-count 0

# RDS の一時停止
aws rds stop-db-cluster \
  --db-cluster-identifier nba-tracker-dev

# MWAA の無効化
terraform apply -var="enable_mwaa=false"
```

#### ロールバック手順

```bash
# 前のバージョンのイメージでデプロイ
docker pull $(terraform output -raw ecr_repository_url):previous-version
docker tag $(terraform output -raw ecr_repository_url):previous-version \
  $(terraform output -raw ecr_repository_url):latest
docker push $(terraform output -raw ecr_repository_url):latest

# ECS サービスの更新
aws ecs update-service \
  --cluster nba-tracker-dev \
  --service nba-tracker-dev \
  --force-new-deployment
```

## コスト最適化

### コスト分析

```bash
# Cost Explorer でのコスト確認
aws ce get-cost-and-usage \
  --time-period Start=2024-01-01,End=2024-01-31 \
  --granularity DAILY \
  --metrics "UnblendedCost" \
  --group-by Type=DIMENSION,Key=SERVICE
```

### 開発環境の自動停止

#### Lambda 関数による自動停止（例）

```python
import boto3
import os

def lambda_handler(event, context):
    ecs = boto3.client('ecs')
    rds = boto3.client('rds')
    
    # ECS サービスの停止
    ecs.update_service(
        cluster='nba-tracker-dev',
        service='nba-tracker-dev',
        desiredCount=0
    )
    
    # RDS の停止
    rds.stop_db_cluster(
        DBClusterIdentifier='nba-tracker-dev'
    )
    
    return {
        'statusCode': 200,
        'body': 'Resources stopped successfully'
    }
```

#### EventBridge ルールの設定

```bash
# 毎日午後10時に停止
aws events put-rule \
  --name nba-tracker-dev-stop \
  --schedule-expression "cron(0 13 * * ? *)"

# 毎日午前8時に起動
aws events put-rule \
  --name nba-tracker-dev-start \
  --schedule-expression "cron(0 23 * * ? *)"
```

### リソースの最適化

1. **未使用リソースの削除**
   ```bash
   # 未使用の EBS ボリューム
   aws ec2 describe-volumes \
     --filters "Name=status,Values=available"
   
   # 古いスナップショット
   aws rds describe-db-cluster-snapshots \
     --db-cluster-identifier nba-tracker-dev \
     --query 'reverse(sort_by(DBClusterSnapshots, &SnapshotCreateTime))[10:]'
   ```

2. **リザーブドインスタンスの検討**
   - 本番環境では Reserved Capacity を購入
   - Savings Plans の適用

3. **S3 ライフサイクルポリシー**
   ```bash
   aws s3api put-bucket-lifecycle-configuration \
     --bucket nba-tracker-dev-logs \
     --lifecycle-configuration file://lifecycle.json
   ```

## セキュリティ運用

### 定期的なセキュリティ監査

1. **IAM アクセスの確認**
   ```bash
   # 未使用の認証情報を確認
   aws iam generate-credential-report
   aws iam get-credential-report
   ```

2. **セキュリティグループの監査**
   ```bash
   # 0.0.0.0/0 からのアクセスを許可しているルール
   aws ec2 describe-security-groups \
     --filters "Name=ip-permission.cidr,Values=0.0.0.0/0"
   ```

3. **Secrets のローテーション**
   ```bash
   # データベースパスワードのローテーション
   aws secretsmanager rotate-secret \
     --secret-id nba-tracker-dev-db-password
   ```

### コンプライアンス

1. **AWS Config ルールの設定**
2. **CloudTrail ログの有効化**
3. **定期的な脆弱性スキャン**

## メンテナンス作業

### 定期メンテナンスのチェックリスト

- [ ] CloudWatch Logs の古いログを S3 にアーカイブ
- [ ] 未使用の Docker イメージを ECR から削除
- [ ] RDS の自動バックアップの確認
- [ ] セキュリティパッチの適用
- [ ] Terraform モジュールの更新
- [ ] 依存パッケージの更新

### メンテナンスウィンドウ

```bash
# RDS のメンテナンスウィンドウ設定
aws rds modify-db-cluster \
  --db-cluster-identifier nba-tracker-dev \
  --preferred-maintenance-window "sun:15:00-sun:16:00" \
  --apply-immediately
```