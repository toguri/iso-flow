# NBA Trade Tracker - Development Environment

terraform {
  required_version = ">= 1.5.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
  
  default_tags {
    tags = {
      Environment = "dev"
      Project     = "nba-trade-tracker"
      ManagedBy   = "terraform"
    }
  }
}

# Data source for availability zones
data "aws_availability_zones" "available" {
  state = "available"
}

# VPC Module
module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
  version = "5.5.1"

  name = "${var.project_name}-vpc-${var.environment}"
  cidr = var.vpc_cidr

  azs              = data.aws_availability_zones.available.names
  private_subnets  = var.private_subnet_cidrs
  public_subnets   = var.public_subnet_cidrs
  database_subnets = var.database_subnet_cidrs

  enable_nat_gateway     = true
  single_nat_gateway     = true  # Cost optimization for dev
  enable_dns_hostnames   = true
  enable_dns_support     = true

  # VPC Endpoints for cost optimization
  enable_s3_endpoint = true

  tags = {
    Name = "${var.project_name}-vpc-${var.environment}"
  }
}

# RDS Module
module "rds" {
  source = "../../modules/rds"

  project_name = var.project_name
  environment  = var.environment
  
  db_name     = var.db_name
  db_username = var.db_username
  db_password = var.db_password
  
  vpc_id                  = module.vpc.vpc_id
  database_subnet_ids     = module.vpc.database_subnets
  allowed_security_groups = [module.ecs.ecs_security_group_id]
}

# S3 Module
module "s3" {
  source = "../../modules/s3"

  project_name = var.project_name
  environment  = var.environment
}

# ECS Module
module "ecs" {
  source = "../../modules/ecs"

  project_name = var.project_name
  environment  = var.environment
  
  vpc_id             = module.vpc.vpc_id
  private_subnet_ids = module.vpc.private_subnets
  public_subnet_ids  = module.vpc.public_subnets
  
  container_image = var.container_image
  container_port  = var.container_port
  
  # Pass database connection info
  database_endpoint = module.rds.cluster_endpoint
  database_name     = var.db_name
}

# MWAA Module
module "mwaa" {
  source = "../../modules/mwaa"

  project_name = var.project_name
  environment  = var.environment
  
  vpc_id             = module.vpc.vpc_id
  private_subnet_ids = slice(module.vpc.private_subnets, 0, 2)  # MWAA requires exactly 2 subnets
  
  dag_s3_bucket    = module.s3.mwaa_dag_bucket_name
  results_s3_bucket = module.s3.mwaa_results_bucket_name
  
  # Airflow configuration
  airflow_version = var.airflow_version
  environment_class = var.mwaa_environment_class
  
  # Pass connection info for Airflow to access backend
  backend_api_endpoint = module.ecs.api_endpoint
}