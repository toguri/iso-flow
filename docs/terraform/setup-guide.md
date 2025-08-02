# Terraformセットアップガイド

NBA Trade TrackerのインフラストラクチャをTerraformでセットアップする手順を説明します。

## 前提条件

### 必要なツール

1. **Terraform**
   ```bash
   # macOS
   brew install terraform
   
   # バージョン確認（v1.5.0以上）
   terraform version
   ```

2. **AWS CLI**
   ```bash
   # macOS
   brew install awscli
   
   # 設定
   aws configure
   ```

3. **その他のツール**
   - Docker（ECRへのイメージプッシュ用）
   - jq（JSON処理用）
   - Git

### AWS アカウントの準備

1. **IAM ユーザーの作成**
   - プログラムによるアクセスを有効化
   - 必要な権限ポリシーをアタッチ（下記参照）

2. **必要な IAM 権限**
   ```json
   {
     "Version": "2012-10-17",
     "Statement": [
       {
         "Effect": "Allow",
         "Action": [
           "ec2:*",
           "ecs:*",
           "ecr:*",
           "elasticloadbalancing:*",
           "rds:*",
           "s3:*",
           "cloudfront:*",
           "iam:*",
           "logs:*",
           "secretsmanager:*",
           "ssm:*",
           "airflow:*"
         ],
         "Resource": "*"
       }
     ]
   }
   ```

## 初期セットアップ

### 1. リポジトリのクローン

```bash
git clone https://github.com/your-org/nba-trade-tracker.git
cd nba-trade-tracker/terraform/environments/dev
```

### 2. Terraform バックエンドの設定

#### S3 バケットの作成（Terraform state 保存用）

```bash
# S3バケットの作成
aws s3 mb s3://your-terraform-state-bucket --region ap-northeast-1

# バージョニングの有効化
aws s3api put-bucket-versioning \
  --bucket your-terraform-state-bucket \
  --versioning-configuration Status=Enabled

# 暗号化の有効化
aws s3api put-bucket-encryption \
  --bucket your-terraform-state-bucket \
  --server-side-encryption-configuration '{
    "Rules": [{
      "ApplyServerSideEncryptionByDefault": {
        "SSEAlgorithm": "AES256"
      }
    }]
  }'
```

#### backend.tf の作成

```hcl
# terraform/environments/dev/backend.tf
terraform {
  backend "s3" {
    bucket  = "your-terraform-state-bucket"
    key     = "nba-trade-tracker/dev/terraform.tfstate"
    region  = "ap-northeast-1"
    encrypt = true
  }
}
```

### 3. 変数の設定

#### terraform.tfvars の作成

```bash
cp terraform.tfvars.example terraform.tfvars
```

#### terraform.tfvars の編集

```hcl
# terraform/environments/dev/terraform.tfvars

# プロジェクト基本設定
project_name = "nba-tracker"
environment  = "dev"
region       = "ap-northeast-1"

# データベース設定
db_password = "your-secure-password-here"  # 最低8文字、英数字記号を含む

# ドメイン設定（オプション）
domain_name = "nba-tracker.example.com"  # カスタムドメインを使用する場合

# 機能フラグ
enable_mwaa = false  # 初回はfalseにして、基本インフラのみデプロイ
```

### 4. Terraform の初期化

```bash
# 初期化
terraform init

# 出力例
Initializing modules...
Initializing the backend...
Initializing provider plugins...
Terraform has been successfully initialized!
```

## デプロイ手順

### 1. 基本インフラストラクチャのデプロイ

```bash
# プランの確認
terraform plan

# デプロイの実行
terraform apply
```

確認プロンプトで `yes` を入力してデプロイを開始します。

初回デプロイには約15-20分かかります。

### 2. 出力値の確認

```bash
# 重要な出力値を表示
terraform output

# 出力例
alb_dns_name = "nba-tracker-dev-alb-123456.ap-northeast-1.elb.amazonaws.com"
ecr_repository_url = "123456789012.dkr.ecr.ap-northeast-1.amazonaws.com/nba-tracker-dev"
frontend_bucket_name = "nba-tracker-dev-frontend-bucket"
cloudfront_distribution_id = "E1234567890ABC"
```

### 3. ECR へのログインとイメージプッシュ

```bash
# ECR にログイン
aws ecr get-login-password --region ap-northeast-1 | \
  docker login --username AWS --password-stdin \
  $(terraform output -raw ecr_repository_url | cut -d'/' -f1)

# バックエンドのビルドとプッシュ
cd ../../../backend
docker build -t nba-trade-tracker .
docker tag nba-trade-tracker:latest $(cd ../terraform/environments/dev && terraform output -raw ecr_repository_url):latest
docker push $(cd ../terraform/environments/dev && terraform output -raw ecr_repository_url):latest
```

### 4. データベースマイグレーション

```bash
# データベース URL の取得
cd ../terraform/environments/dev
export DATABASE_URL=$(terraform output -raw database_url)

# マイグレーションの実行
cd ../../../backend
sqlx migrate run
```

### 5. フロントエンドのデプロイ

```bash
cd ../frontend

# ビルド
./gradlew browserProductionWebpack

# S3 へのアップロード
aws s3 sync build/distributions/ \
  s3://$(cd ../terraform/environments/dev && terraform output -raw frontend_bucket_name)/ \
  --delete

# CloudFront のキャッシュ無効化
aws cloudfront create-invalidation \
  --distribution-id $(cd ../terraform/environments/dev && terraform output -raw cloudfront_distribution_id) \
  --paths "/*"
```

### 6. MWAA の有効化（オプション）

基本インフラが正常に動作することを確認後：

```bash
cd ../terraform/environments/dev

# terraform.tfvars を編集
# enable_mwaa = true

# 変更を適用
terraform apply

# DAG のアップロード
cd ../../../airflow
aws s3 sync dags/ s3://$(cd ../terraform/environments/dev && terraform output -raw mwaa_dag_bucket)/dags/
aws s3 cp requirements.txt s3://$(cd ../terraform/environments/dev && terraform output -raw mwaa_dag_bucket)/
```

## 動作確認

### 1. ALB ヘルスチェック

```bash
# ヘルスチェックエンドポイントの確認
curl -i http://$(terraform output -raw alb_dns_name)/health

# 期待される応答
HTTP/1.1 200 OK
Content-Type: application/json
{"status":"healthy","timestamp":"2024-01-01T00:00:00Z"}
```

### 2. GraphQL Playground

ブラウザで以下にアクセス：
```
http://$(terraform output -raw alb_dns_name)/graphql
```

### 3. フロントエンド

CloudFront URL または設定したカスタムドメインにアクセス：
```
https://$(terraform output -raw cloudfront_domain_name)
```

## トラブルシューティング

### Terraform init が失敗する

```bash
# プロバイダーのキャッシュをクリア
rm -rf .terraform
rm .terraform.lock.hcl
terraform init -upgrade
```

### ECS タスクが起動しない

1. CloudWatch Logs でタスクログを確認
2. ECR イメージが正しくプッシュされているか確認
3. タスクロールの権限を確認

### RDS 接続エラー

1. セキュリティグループのルールを確認
2. データベースが起動しているか確認
3. Secrets Manager のシークレットを確認

### MWAA が起動しない

1. S3 バケットのバージョニングが有効か確認
2. VPC に最低2つのプライベートサブネットがあるか確認
3. CloudWatch Logs でエラーを確認

## 環境の更新

### インフラストラクチャの変更

```bash
# 変更内容の確認
terraform plan

# 変更の適用
terraform apply
```

### アプリケーションの更新

```bash
# バックエンドの更新
docker build -t nba-trade-tracker .
docker push $(terraform output -raw ecr_repository_url):latest

# ECS サービスの強制更新
aws ecs update-service \
  --cluster nba-tracker-dev \
  --service nba-tracker-dev \
  --force-new-deployment
```

## 環境の削除

### 注意事項

削除前に以下を確認：
- データベースのバックアップを取得
- S3 バケットの内容を必要に応じて保存
- CloudWatch Logs を必要に応じてエクスポート

### 削除手順

```bash
# S3 バケットを空にする
aws s3 rm s3://$(terraform output -raw frontend_bucket_name) --recursive
aws s3 rm s3://$(terraform output -raw mwaa_dag_bucket) --recursive

# Terraform リソースの削除
terraform destroy

# 確認プロンプトで yes を入力
```

## ベストプラクティス

1. **バージョン管理**
   - Terraform のバージョンを固定
   - プロバイダーのバージョンを固定
   - .terraform.lock.hcl をコミット

2. **状態管理**
   - リモートバックエンドを使用
   - state ファイルを直接編集しない
   - terraform refresh を定期的に実行

3. **セキュリティ**
   - 機密情報を terraform.tfvars に含めない
   - Secrets Manager を活用
   - 最小権限の原則を適用

4. **コスト管理**
   - 不要なリソースは削除
   - 開発環境は夜間停止を検討
   - Cost Explorer でコストを監視