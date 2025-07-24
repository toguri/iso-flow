# General Variables
variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "ap-northeast-1"
}

variable "project_name" {
  description = "Project name"
  type        = string
  default     = "nba-trade-tracker"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "dev"
}

# VPC Variables
variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"
}

variable "private_subnet_cidrs" {
  description = "CIDR blocks for private subnets"
  type        = list(string)
  default     = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
}

variable "public_subnet_cidrs" {
  description = "CIDR blocks for public subnets"
  type        = list(string)
  default     = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]
}

variable "database_subnet_cidrs" {
  description = "CIDR blocks for database subnets"
  type        = list(string)
  default     = ["10.0.201.0/24", "10.0.202.0/24", "10.0.203.0/24"]
}

# RDS Variables
variable "db_name" {
  description = "Database name"
  type        = string
  default     = "nba_trades"
}

variable "db_username" {
  description = "Database username"
  type        = string
  default     = "admin"
}

variable "db_password" {
  description = "Database password"
  type        = string
  sensitive   = true
}

# ECS Variables
variable "container_image" {
  description = "Docker image for the backend container"
  type        = string
  default     = "nba-trade-tracker:latest"
}

variable "container_port" {
  description = "Port exposed by the container"
  type        = number
  default     = 8000
}

# MWAA Variables
variable "airflow_version" {
  description = "Apache Airflow version"
  type        = string
  default     = "2.8.1"
}

variable "mwaa_environment_class" {
  description = "MWAA environment class"
  type        = string
  default     = "mw1.small"
}