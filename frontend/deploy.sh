#!/bin/bash
# Frontend deployment script for S3 + CloudFront

set -e

# 環境変数の確認
if [ -z "$S3_BUCKET" ]; then
    echo "Error: S3_BUCKET environment variable is not set"
    exit 1
fi

if [ -z "$CLOUDFRONT_DISTRIBUTION_ID" ]; then
    echo "Error: CLOUDFRONT_DISTRIBUTION_ID environment variable is not set"
    exit 1
fi

if [ -z "$API_URL" ]; then
    echo "Error: API_URL environment variable is not set"
    exit 1
fi

echo "Building Kotlin/JS application..."

# API URLを環境変数として設定
export REACT_APP_API_URL=$API_URL

# ビルド実行
./gradlew clean
./gradlew jsBrowserProductionWebpack

# ビルド成果物の確認
if [ ! -d "build/distributions" ]; then
    echo "Error: Build directory not found"
    exit 1
fi

echo "Uploading to S3..."

# S3に同期
aws s3 sync build/distributions/ s3://$S3_BUCKET/ \
    --delete \
    --cache-control "public, max-age=3600" \
    --exclude "*.map"

# index.htmlは短いキャッシュ時間を設定
aws s3 cp build/distributions/index.html s3://$S3_BUCKET/index.html \
    --cache-control "public, max-age=300"

echo "Invalidating CloudFront cache..."

# CloudFrontのキャッシュを無効化
aws cloudfront create-invalidation \
    --distribution-id $CLOUDFRONT_DISTRIBUTION_ID \
    --paths "/*"

echo "Deployment completed successfully!"