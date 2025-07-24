terraform {
  required_version = ">= 1.5.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }

  backend "s3" {
    # バックエンドの設定は backend.tf で管理
  }
}

provider "aws" {
  region = var.aws_region

  default_tags {
    tags = var.tags
  }
}

# VPC Module
module "vpc" {
  source = "../../modules/vpc"

  project_name = var.project_name
  environment  = var.environment
  vpc_cidr     = var.vpc_cidr
  azs          = var.azs
  tags         = var.tags
}

# RDS Aurora Serverless v2
module "rds" {
  source = "../../modules/rds"

  project_name      = var.project_name
  environment       = var.environment
  db_name          = var.db_name
  db_username      = var.db_username
  db_password      = var.db_password
  vpc_id           = module.vpc.vpc_id
  database_subnets = module.vpc.database_subnets
  tags             = var.tags
}

# ECS Fargate for Backend
module "ecs" {
  source = "../../modules/ecs"

  project_name    = var.project_name
  environment     = var.environment
  vpc_id          = module.vpc.vpc_id
  private_subnets = module.vpc.private_subnets
  public_subnets  = module.vpc.public_subnets
  task_cpu        = var.ecs_task_cpu
  task_memory     = var.ecs_task_memory
  database_url    = module.rds.database_url
  tags            = var.tags
}

# S3 for Frontend and MWAA DAGs
module "s3" {
  source = "../../modules/s3"

  project_name = var.project_name
  environment  = var.environment
  tags         = var.tags
}

# MWAA (Managed Apache Airflow)
module "mwaa" {
  source = "../../modules/mwaa"

  project_name          = var.project_name
  environment           = var.environment
  vpc_id                = module.vpc.vpc_id
  private_subnets       = module.vpc.private_subnets
  dag_s3_bucket         = module.s3.mwaa_dag_bucket
  environment_class     = var.mwaa_environment_class
  min_workers          = var.mwaa_min_workers
  max_workers          = var.mwaa_max_workers
  backend_api_endpoint  = module.ecs.api_endpoint
  database_secret_arn   = module.rds.database_secret_arn
  tags                  = var.tags
}