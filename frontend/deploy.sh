#!/bin/bash

# Frontend deployment script for NBA Trade Tracker

set -e

echo "🏀 NBA Trade Tracker Frontend Deployment"
echo "========================================"

# Check if we're in the frontend directory
if [ ! -f "build.gradle.kts" ]; then
    echo "❌ Error: This script must be run from the frontend directory"
    exit 1
fi

# Get S3 bucket name from Terraform output
echo "📦 Getting deployment information..."
cd ../terraform/environments/dev
FRONTEND_BUCKET=$(terraform output -raw frontend_bucket_name 2>/dev/null)
CLOUDFRONT_ID=$(terraform output -raw cloudfront_distribution_id 2>/dev/null)

if [ -z "$FRONTEND_BUCKET" ] || [ -z "$CLOUDFRONT_ID" ]; then
    echo "❌ Error: Could not get deployment information from Terraform"
    echo "Make sure Terraform has been applied successfully"
    exit 1
fi

cd ../../../frontend

# Build the frontend
echo "🔨 Building frontend..."
./gradlew build

# Check if build was successful
if [ ! -d "build/dist/js/productionExecutable" ]; then
    echo "❌ Error: Build failed - output directory not found"
    exit 1
fi

# Deploy to S3
echo "☁️  Uploading to S3..."
aws s3 sync build/dist/js/productionExecutable/ s3://$FRONTEND_BUCKET/ \
    --delete \
    --cache-control "public, max-age=3600"

# Invalidate CloudFront cache
echo "🔄 Invalidating CloudFront cache..."
aws cloudfront create-invalidation \
    --distribution-id $CLOUDFRONT_ID \
    --paths "/*" \
    --query 'Invalidation.Id' \
    --output text

echo "✅ Deployment complete!"
echo ""
echo "Your frontend is available at:"
cd ../terraform/environments/dev
terraform output frontend_url