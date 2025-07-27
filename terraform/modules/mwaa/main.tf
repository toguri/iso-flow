# Amazon MWAA (Managed Apache Airflow) Module

terraform {
  required_version = ">= 1.5.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

locals {
  name = "${var.project_name}-${var.environment}"
}

# MWAA Security Group
resource "aws_security_group" "mwaa" {
  name        = "${local.name}-mwaa-sg"
  description = "Security group for MWAA environment"
  vpc_id      = var.vpc_id

  ingress {
    from_port = 0
    to_port   = 0
    protocol  = "-1"
    self      = true
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(var.tags, {
    Name = "${local.name}-mwaa-sg"
  })
}

# IAM Role for MWAA Execution
resource "aws_iam_role" "mwaa_execution" {
  name = "${local.name}-mwaa-execution-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = [
            "airflow-env.amazonaws.com",
            "airflow.amazonaws.com"
          ]
        }
      }
    ]
  })

  tags = var.tags
}

# IAM Policy for MWAA
resource "aws_iam_role_policy" "mwaa_execution" {
  name = "${local.name}-mwaa-execution-policy"
  role = aws_iam_role.mwaa_execution.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "airflow:PublishMetrics"
        ]
        Resource = "arn:aws:airflow:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:environment/${local.name}"
      },
      {
        Effect = "Allow"
        Action = [
          "s3:ListBucket",
          "s3:GetObject",
          "s3:GetObjectVersion"
        ]
        Resource = [
          var.dag_s3_bucket_arn,
          "${var.dag_s3_bucket_arn}/*"
        ]
      },
      {
        Effect = "Allow"
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ]
        Resource = "arn:aws:logs:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:log-group:airflow-${local.name}-*"
      },
      {
        Effect = "Allow"
        Action = [
          "logs:DescribeLogGroups"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "s3:GetAccountPublicAccessBlock"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "secretsmanager:GetSecretValue"
        ]
        Resource = var.database_secret_arn
      },
      {
        Effect = "Allow"
        Action = [
          "kms:Decrypt",
          "kms:DescribeKey",
          "kms:GenerateDataKey*",
          "kms:CreateGrant"
        ]
        Resource = "arn:aws:kms:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:key/*"
        Condition = {
          StringLike = {
            "kms:ViaService" = [
              "sqs.${data.aws_region.current.name}.amazonaws.com",
              "s3.${data.aws_region.current.name}.amazonaws.com"
            ]
          }
        }
      }
    ]
  })
}

# Attach AWS managed policy
resource "aws_iam_role_policy_attachment" "mwaa_execution" {
  role       = aws_iam_role.mwaa_execution.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonMWAAServiceRolePolicy"
}

# MWAA Environment
resource "aws_mwaa_environment" "main" {
  name               = local.name
  airflow_version    = "2.8.1"
  environment_class  = var.environment_class
  execution_role_arn = aws_iam_role.mwaa_execution.arn

  source_bucket_arn    = var.dag_s3_bucket_arn
  dag_s3_path          = "dags/"
  requirements_s3_path = "requirements.txt"

  min_workers = var.min_workers
  max_workers = var.max_workers

  network_configuration {
    security_group_ids = [aws_security_group.mwaa.id]
    subnet_ids         = var.private_subnets
  }

  logging_configuration {
    dag_processing_logs {
      enabled   = true
      log_level = "INFO"
    }

    scheduler_logs {
      enabled   = true
      log_level = "INFO"
    }

    task_logs {
      enabled   = true
      log_level = "INFO"
    }

    webserver_logs {
      enabled   = true
      log_level = "INFO"
    }

    worker_logs {
      enabled   = true
      log_level = "INFO"
    }
  }

  airflow_configuration_options = {
    "core.default_timezone"   = "Asia/Tokyo"
    "webserver.expose_config" = "true"
    "core.load_examples"      = "false"
  }

  tags = merge(var.tags, {
    Name = local.name
  })
}

# Create Airflow connections via AWS Systems Manager Parameter Store
resource "aws_ssm_parameter" "backend_api_endpoint" {
  name  = "/mwaa/${local.name}/connections/backend_api_endpoint"
  type  = "SecureString"
  value = var.backend_api_endpoint

  tags = var.tags
}

resource "aws_ssm_parameter" "aurora_connection" {
  name = "/mwaa/${local.name}/connections/aurora_postgres"
  type = "SecureString"
  value = jsonencode({
    conn_type = "postgres"
    host      = split(":", replace(var.backend_api_endpoint, "http://", ""))[0]
    schema    = "nba_trades"
    login     = "admin"
    port      = 5432
  })

  tags = var.tags
}

# Data sources
data "aws_region" "current" {}
data "aws_caller_identity" "current" {}

# S3 bucket for storing DAG results (optional)
resource "aws_s3_bucket" "mwaa_results" {
  bucket = "${local.name}-mwaa-results"

  tags = merge(var.tags, {
    Name = "${local.name}-mwaa-results"
    Type = "MWAA-Results"
  })
}

resource "aws_s3_bucket_public_access_block" "mwaa_results" {
  bucket = aws_s3_bucket.mwaa_results.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}