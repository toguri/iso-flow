#!/bin/bash

# Setup Terraform Backend Resources for Production
# This script creates S3 bucket and DynamoDB table for Terraform state management

set -e

REGION="ap-northeast-1"
BUCKET_NAME="iso-flow-terraform-state-prod"
DYNAMODB_TABLE="iso-flow-terraform-lock-prod"
PROJECT="iso-flow"
ENVIRONMENT="prod"

echo "🚀 Setting up Terraform backend resources for production..."

# Check if AWS CLI is configured
if ! aws sts get-caller-identity > /dev/null 2>&1; then
    echo "❌ AWS CLI is not configured. Please run 'aws configure' first."
    exit 1
fi

ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)
echo "📍 Using AWS Account: $ACCOUNT_ID"
echo "📍 Region: $REGION"

# Create S3 bucket for Terraform state
echo "📦 Creating S3 bucket: $BUCKET_NAME"
if aws s3api head-bucket --bucket "$BUCKET_NAME" 2>/dev/null; then
    echo "✅ S3 bucket already exists"
else
    aws s3api create-bucket \
        --bucket "$BUCKET_NAME" \
        --region "$REGION" \
        --create-bucket-configuration LocationConstraint="$REGION"
    
    # Enable versioning
    aws s3api put-bucket-versioning \
        --bucket "$BUCKET_NAME" \
        --versioning-configuration Status=Enabled
    
    # Enable encryption
    aws s3api put-bucket-encryption \
        --bucket "$BUCKET_NAME" \
        --server-side-encryption-configuration '{
            "Rules": [{
                "ApplyServerSideEncryptionByDefault": {
                    "SSEAlgorithm": "AES256"
                }
            }]
        }'
    
    # Block public access
    aws s3api put-public-access-block \
        --bucket "$BUCKET_NAME" \
        --public-access-block-configuration \
        "BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true"
    
    # Add tags
    aws s3api put-bucket-tagging \
        --bucket "$BUCKET_NAME" \
        --tagging "{
            \"TagSet\": [
                {\"Key\": \"Project\", \"Value\": \"$PROJECT\"},
                {\"Key\": \"Environment\", \"Value\": \"$ENVIRONMENT\"},
                {\"Key\": \"ManagedBy\", \"Value\": \"Terraform\"},
                {\"Key\": \"Purpose\", \"Value\": \"TerraformState\"}
            ]
        }"
    
    echo "✅ S3 bucket created and configured"
fi

# Create DynamoDB table for state locking
echo "🔒 Creating DynamoDB table: $DYNAMODB_TABLE"
if aws dynamodb describe-table --table-name "$DYNAMODB_TABLE" --region "$REGION" > /dev/null 2>&1; then
    echo "✅ DynamoDB table already exists"
else
    aws dynamodb create-table \
        --table-name "$DYNAMODB_TABLE" \
        --attribute-definitions AttributeName=LockID,AttributeType=S \
        --key-schema AttributeName=LockID,KeyType=HASH \
        --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5 \
        --region "$REGION" \
        --tags \
            Key=Project,Value="$PROJECT" \
            Key=Environment,Value="$ENVIRONMENT" \
            Key=ManagedBy,Value=Terraform \
            Key=Purpose,Value=TerraformStateLock
    
    # Wait for table to be active
    echo "⏳ Waiting for DynamoDB table to be active..."
    aws dynamodb wait table-exists --table-name "$DYNAMODB_TABLE" --region "$REGION"
    echo "✅ DynamoDB table created"
fi

echo ""
echo "✅ Backend resources setup complete!"
echo ""
echo "📝 Backend configuration:"
echo "   S3 Bucket: $BUCKET_NAME"
echo "   DynamoDB Table: $DYNAMODB_TABLE"
echo "   Region: $REGION"
echo ""
echo "🔐 Make sure your backend.tf contains:"
echo "   bucket = \"$BUCKET_NAME\""
echo "   dynamodb_table = \"$DYNAMODB_TABLE\""
echo "   region = \"$REGION\""
echo ""
echo "Next steps:"
echo "1. Update backend.tf with the correct values (if needed)"
echo "2. Run: cd /Users/a13694/usr/src/iso-flow/terraform/environments/prod"
echo "3. Run: terraform init"
echo "4. Run: terraform plan"