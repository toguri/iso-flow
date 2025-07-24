# Terraform Setup Guide for ISO-Flow

This guide will help you set up the AWS infrastructure for the ISO-Flow project using Terraform.

## Prerequisites

1. **AWS CLI** configured with appropriate credentials
   ```bash
   aws configure
   ```

2. **Terraform** installed (version >= 1.5.0)
   ```bash
   brew install terraform  # macOS
   # or download from https://www.terraform.io/downloads
   ```

3. **AWS Account** with appropriate permissions to create:
   - VPC and networking resources
   - RDS Aurora Serverless v2
   - ECS Fargate and ALB
   - S3 buckets
   - IAM roles and policies
   - CloudWatch logs
   - Amazon MWAA (optional)

## Architecture Overview

The Terraform configuration creates the following AWS resources:

- **Networking**: VPC with public, private, and database subnets across multiple AZs
- **Database**: RDS Aurora PostgreSQL Serverless v2 (cost-optimized for dev)
- **Compute**: ECS Fargate for containerized applications
- **Load Balancing**: Application Load Balancer (ALB)
- **Storage**: S3 buckets for frontend hosting and MWAA
- **Orchestration**: Amazon MWAA for Apache Airflow (optional)
- **Monitoring**: CloudWatch logs and Performance Insights

## Setup Steps

### 1. Backend Configuration

First, create the backend resources for storing Terraform state:

```bash
# Create S3 bucket for state
aws s3api create-bucket \
  --bucket your-terraform-state-bucket \
  --region ap-northeast-1 \
  --create-bucket-configuration LocationConstraint=ap-northeast-1

# Enable versioning
aws s3api put-bucket-versioning \
  --bucket your-terraform-state-bucket \
  --versioning-configuration Status=Enabled

# Create DynamoDB table for state locking
aws dynamodb create-table \
  --table-name terraform-state-lock \
  --attribute-definitions AttributeName=LockID,AttributeType=S \
  --key-schema AttributeName=LockID,KeyType=HASH \
  --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5 \
  --region ap-northeast-1
```

### 2. Configure Terraform Backend

```bash
cd terraform/environments/dev
cp backend.tf.example backend.tf
```

Edit `backend.tf` and update the bucket name and other parameters.

### 3. Create Variables File

```bash
cp terraform.tfvars.example terraform.tfvars
```

Edit `terraform.tfvars` and update:
- `database_password` - Use a strong password or AWS Secrets Manager
- `ecr_repository_url` - Replace with your ECR repository URL
- Optional: domain and certificate configuration

### 4. Create ECR Repository

Before deploying, create an ECR repository for your container images:

```bash
aws ecr create-repository \
  --repository-name iso-flow \
  --region ap-northeast-1
```

### 5. Initialize and Deploy

```bash
# Initialize Terraform
terraform init

# Review the plan
terraform plan

# Apply the configuration
terraform apply
```

## Cost Optimization for Development

The default configuration is optimized for development with:

- **Aurora Serverless v2**: 0.5-1 ACU (minimum capacity)
- **ECS Fargate**: 0.25 vCPU, 512MB memory
- **Single AZ deployment** for non-critical resources
- **Minimal log retention** (7 days)

### Estimated Monthly Costs (Dev Environment)

- RDS Aurora Serverless v2: ~$50-100 (depending on usage)
- ECS Fargate: ~$10-20 (single task)
- ALB: ~$20
- VPC/NAT Gateway: ~$45
- S3/CloudWatch: ~$5-10

**Total**: ~$130-200/month

### Cost-Saving Tips

1. **Stop Aurora cluster** when not in use:
   ```bash
   aws rds stop-db-cluster --db-cluster-identifier iso-flow-dev
   ```

2. **Scale down ECS tasks** to 0 when not needed:
   ```bash
   aws ecs update-service \
     --cluster iso-flow-dev \
     --service your-service-name \
     --desired-count 0
   ```

3. **Use AWS Free Tier** resources when possible

4. **Consider using RDS Proxy** to reduce database connections

## Module Structure

### VPC Module

The VPC module (`terraform/modules/vpc`) creates:
- VPC with configurable CIDR
- Public subnets (for ALB)
- Private subnets (for ECS tasks)
- Database subnets (for RDS)
- Internet Gateway and NAT Gateways
- Route tables

## Deployment Workflow

### Backend Application

1. Build and push Docker image:
   ```bash
   # Build image
   docker build -t iso-flow-backend .
   
   # Tag for ECR
   docker tag iso-flow-backend:latest YOUR_ACCOUNT_ID.dkr.ecr.ap-northeast-1.amazonaws.com/iso-flow:latest
   
   # Push to ECR
   aws ecr get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin YOUR_ACCOUNT_ID.dkr.ecr.ap-northeast-1.amazonaws.com
   docker push YOUR_ACCOUNT_ID.dkr.ecr.ap-northeast-1.amazonaws.com/iso-flow:latest
   ```

2. Create ECS task definition and service (add to main.tf or create separately)

### Frontend Application

Deploy static files to S3:

```bash
# Build your frontend
npm run build

# Sync to S3
aws s3 sync ./dist s3://iso-flow-dev-frontend --delete
```

## Security Best Practices

1. **Secrets Management**:
   - Use AWS Secrets Manager for database passwords
   - Use AWS Systems Manager Parameter Store for configuration
   - Never commit sensitive values to version control

2. **Network Security**:
   - RDS is only accessible from ECS tasks
   - Use security groups as virtual firewalls
   - Enable VPC Flow Logs for monitoring

3. **IAM Roles**:
   - Follow least privilege principle
   - Use separate roles for different services
   - Enable CloudTrail for audit logging

## Monitoring and Logging

- **CloudWatch Logs**: All ECS tasks and RDS logs
- **Performance Insights**: Enabled for RDS
- **Container Insights**: Enabled for ECS cluster
- **ALB Access Logs**: Stored in S3

## Troubleshooting

### Common Issues

1. **Terraform state lock error**:
   ```bash
   terraform force-unlock <LOCK_ID>
   ```

2. **ECS tasks failing to start**:
   - Check CloudWatch logs
   - Verify security groups
   - Check task IAM role permissions

3. **Database connection issues**:
   - Verify security group rules
   - Check RDS cluster status
   - Validate connection string in ECS task

## Clean Up

To destroy all resources and avoid charges:

```bash
# Remove all resources
terraform destroy

# Confirm by typing 'yes'
```

**Note**: Ensure you have backups of any important data before destroying resources.

## Next Steps

1. Set up CI/CD pipeline for automated deployments
2. Configure monitoring and alerting
3. Implement auto-scaling for ECS services
4. Set up Amazon MWAA for workflow orchestration
5. Configure CloudFront for frontend CDN

## Support

For issues or questions:
1. Check AWS CloudWatch logs
2. Review Terraform state: `terraform show`
3. Validate configurations: `terraform validate`