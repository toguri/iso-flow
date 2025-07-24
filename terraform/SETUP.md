# Terraform Setup Guide

このガイドでは、NBA Trade TrackerのAWSインフラストラクチャをTerraformでデプロイする手順を説明します。

## 前提条件

1. **Terraform**のインストール（v1.5.0以上）
   ```bash
   brew install terraform
   ```

2. **AWS CLI**のインストールと設定
   ```bash
   brew install awscli
   aws configure
   ```

3. **必要なAWS権限**
   - VPC、サブネット、セキュリティグループの作成
   - RDS Aurora Serverless v2の作成
   - ECS、ECR、ALBの作成
   - S3バケット、CloudFrontの作成
   - Amazon MWAAの作成
   - IAMロール、ポリシーの作成
   - Secrets Managerの使用

## セットアップ手順

### 1. Terraform State用のS3バケットを作成

```bash
# S3バケットを作成（バケット名は一意である必要があります）
aws s3 mb s3://your-terraform-state-bucket-name --region ap-northeast-1

# バージョニングを有効化
aws s3api put-bucket-versioning \
  --bucket your-terraform-state-bucket-name \
  --versioning-configuration Status=Enabled
```

### 2. Backend設定ファイルを作成

```bash
cd terraform/environments/dev
cp backend.tf.example backend.tf
```

`backend.tf`を編集して、作成したS3バケット名を設定：

```hcl
terraform {
  backend "s3" {
    bucket = "your-terraform-state-bucket-name"  # ← ここを変更
    key    = "nba-trade-tracker/dev/terraform.tfstate"
    region = "ap-northeast-1"
  }
}
```

### 3. 変数ファイルを作成

```bash
cp terraform.tfvars.example terraform.tfvars
```

`terraform.tfvars`を編集して、必要な値を設定：

```hcl
# データベースパスワード（強力なパスワードを設定）
db_password = "YourStrongPassword123!"
```

### 4. Terraformの初期化

```bash
terraform init
```

### 5. 実行計画の確認

```bash
terraform plan
```

### 6. インフラストラクチャのデプロイ

```bash
terraform apply
```

確認プロンプトで`yes`を入力してデプロイを開始します。

## デプロイ後の作業

### 1. ECRへのDockerイメージのプッシュ

Terraformの出力から`ecr_repository_url`を取得：

```bash
terraform output ecr_repository_url
```

Dockerイメージをビルドしてプッシュ：

```bash
# ECRにログイン
aws ecr get-login-password --region ap-northeast-1 | \
  docker login --username AWS --password-stdin $(terraform output -raw ecr_repository_url)

# バックエンドディレクトリに移動
cd ../../../backend

# Dockerイメージをビルド
docker build -t nba-trade-tracker .

# タグ付け
docker tag nba-trade-tracker:latest $(terraform output -raw ecr_repository_url):latest

# プッシュ
docker push $(terraform output -raw ecr_repository_url):latest
```

### 2. Airflow DAGsのアップロード

```bash
# Terraformディレクトリに戻る
cd ../terraform/environments/dev

# DAGバケット名を取得
DAG_BUCKET=$(terraform output -raw mwaa_dag_bucket_name)

# Airflowディレクトリに移動
cd ../../../airflow

# DAGsをアップロード
aws s3 sync dags/ s3://$DAG_BUCKET/dags/

# requirements.txtをアップロード
aws s3 cp requirements.txt s3://$DAG_BUCKET/
```

### 3. フロントエンドのデプロイ

```bash
cd ../frontend

# ビルド
./gradlew build

# S3にアップロード
aws s3 sync build/dist/js/productionExecutable/ s3://$(terraform output -raw frontend_bucket_name)/

# CloudFrontのキャッシュをクリア
aws cloudfront create-invalidation \
  --distribution-id $(terraform output -raw cloudfront_distribution_id) \
  --paths "/*"
```

## アクセス情報の確認

デプロイ完了後、以下のコマンドで各サービスのエンドポイントを確認できます：

```bash
# すべての出力を表示
terraform output

# 個別に取得
terraform output backend_api_endpoint    # バックエンドAPI
terraform output frontend_url           # フロントエンドURL
terraform output mwaa_webserver_url     # Airflow Web UI
```

## クリーンアップ

リソースを削除する場合：

```bash
# 削除前に確認
terraform plan -destroy

# リソースを削除
terraform destroy
```

**注意**: S3バケットにオブジェクトが残っている場合は、先に削除する必要があります。

## トラブルシューティング

### Terraform initでエラーが出る場合

- AWS認証情報が正しく設定されているか確認：`aws sts get-caller-identity`
- S3バケットが存在し、アクセス権限があるか確認

### ECSタスクが起動しない場合

- ECRにイメージがプッシュされているか確認
- CloudWatch Logsでエラーログを確認
- タスクロールに必要な権限があるか確認

### MWAAが起動しない場合

- S3バケットのバージョニングが有効か確認
- requirements.txtが正しくアップロードされているか確認
- VPCに少なくとも2つのプライベートサブネットがあるか確認