# デプロイメントガイド

このガイドでは、iso-flowアプリケーションをAWSにデプロイする手順を説明します。

## 前提条件

1. **AWS CLIのインストールと設定**
   ```bash
   aws configure
   ```
   以下の情報が必要です：
   - AWS Access Key ID
   - AWS Secret Access Key
   - Default region name (ap-northeast-1推奨)
   - Default output format (json推奨)

2. **必要なツール**
   - Terraform >= 1.0
   - Docker
   - AWS CLI v2
   - Node.js >= 16 (フロントエンド用)
   - Rust (バックエンドのローカルビルド用)

## デプロイ手順

### 1. Terraformによるインフラストラクチャの構築

1. **terraform.tfvarsの作成**
   ```bash
   cd terraform/environments/dev
   cp terraform.tfvars.example terraform.tfvars
   ```

2. **terraform.tfvarsを編集**
   - `database_password`を安全なパスワードに変更
   - `ecr_repository_url`のYOUR_AWS_ACCOUNT_IDを実際のアカウントIDに変更

3. **Terraformの実行**
   ```bash
   terraform init
   terraform plan
   terraform apply
   ```

### 2. バックエンドのデプロイ

1. **ECRへのログイン**
   ```bash
   aws ecr get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin <YOUR_ACCOUNT_ID>.dkr.ecr.ap-northeast-1.amazonaws.com
   ```

2. **Dockerイメージのビルドとプッシュ**
   ```bash
   cd backend
   docker build -t iso-flow-backend .
   docker tag iso-flow-backend:latest <ECR_REPOSITORY_URL>:latest
   docker push <ECR_REPOSITORY_URL>:latest
   ```

3. **ECSサービスの更新**
   ```bash
   aws ecs update-service --cluster iso-flow-dev-cluster --service iso-flow-dev-backend-service --force-new-deployment
   ```

### 3. フロントエンドのデプロイ

1. **フロントエンドのビルド**
   ```bash
   cd frontend
   ./gradlew jsBrowserDistribution
   ```

2. **S3へのアップロード**
   ```bash
   aws s3 sync build/dist/js/productionExecutable/ s3://<FRONTEND_BUCKET_NAME>/ --delete
   ```

3. **CloudFrontキャッシュの無効化**
   ```bash
   aws cloudfront create-invalidation --distribution-id <DISTRIBUTION_ID> --paths "/*"
   ```

## 環境変数の設定

### バックエンド環境変数 (ECSタスク定義で設定)
- `DATABASE_URL`: RDSのエンドポイント（Secrets Managerから取得）
- `PORT`: 8000
- `HOST`: 0.0.0.0
- `RUST_LOG`: info

### フロントエンド設定
- GraphQLエンドポイント: ALBのDNS名を使用

## 確認手順

1. **バックエンドの動作確認**
   ```bash
   curl http://<ALB_DNS_NAME>/health
   curl http://<ALB_DNS_NAME>/graphql
   ```

2. **フロントエンドの動作確認**
   - CloudFront URLまたはS3静的ウェブサイトURLにアクセス

3. **ログの確認**
   ```bash
   aws logs tail /ecs/iso-flow-dev --follow
   ```

## トラブルシューティング

### ECSタスクが起動しない場合
1. CloudWatch Logsでエラーを確認
2. タスク定義の環境変数を確認
3. セキュリティグループの設定を確認

### データベース接続エラー
1. RDSのセキュリティグループを確認
2. Secrets Managerの権限を確認
3. DATABASE_URLの形式を確認

### フロントエンドが表示されない場合
1. S3バケットポリシーを確認
2. CloudFrontのオリジン設定を確認
3. CORSの設定を確認

## コスト最適化のヒント

1. 開発環境では以下の設定を使用：
   - Aurora Serverless v2: 最小0.5 ACU
   - ECS Fargate: 0.25 vCPU, 512MB メモリ
   - NAT Gateway: シングルAZ構成

2. 不要な時はリソースを停止：
   ```bash
   # ECSサービスのタスク数を0に設定
   aws ecs update-service --cluster iso-flow-dev-cluster --service iso-flow-dev-backend-service --desired-count 0
   
   # Aurora Serverlessは自動的にスケールダウン
   ```

## 本番環境への移行

1. `terraform/environments/prod`ディレクトリを使用
2. 以下を考慮：
   - マルチAZ構成
   - 自動スケーリングの設定
   - バックアップとディザスタリカバリ
   - モニタリングとアラート
   - セキュリティの強化（WAF、Shield等）