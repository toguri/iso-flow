variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "ap-northeast-1"
}

variable "project_name" {
  description = "Project name"
  type        = string
  default     = "iso-flow"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "dev"
}

variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"
}

# RDS Aurora Variables
variable "aurora_engine_version" {
  description = "Aurora PostgreSQL engine version"
  type        = string
  default     = "15.4"
}

variable "aurora_instance_count" {
  description = "Number of Aurora instances"
  type        = number
  default     = 1
}

variable "aurora_min_capacity" {
  description = "Minimum capacity for Aurora Serverless v2"
  type        = number
  default     = 0.5
}

variable "aurora_max_capacity" {
  description = "Maximum capacity for Aurora Serverless v2"
  type        = number
  default     = 1
}

variable "database_name" {
  description = "Database name"
  type        = string
  default     = "isoflow"
}

variable "database_username" {
  description = "Database username"
  type        = string
  default     = "isoflow_admin"
  sensitive   = true
}

variable "database_password" {
  description = "Database password"
  type        = string
  sensitive   = true
}

variable "backup_retention_days" {
  description = "Backup retention period in days"
  type        = number
  default     = 7
}

# ECS Variables
variable "ecs_task_cpu" {
  description = "CPU units for ECS task"
  type        = string
  default     = "256"
}

variable "ecs_task_memory" {
  description = "Memory for ECS task in MB"
  type        = string
  default     = "512"
}

variable "ecs_desired_count" {
  description = "Desired number of ECS tasks"
  type        = number
  default     = 1
}

variable "container_port" {
  description = "Port exposed by the container"
  type        = number
  default     = 8080
}


variable "container_image_tag" {
  description = "Container image tag"
  type        = string
  default     = "latest"
}

# Logging
variable "log_retention_days" {
  description = "CloudWatch logs retention in days"
  type        = number
  default     = 7
}

# Domain and Certificate (optional)
variable "domain_name" {
  description = "Domain name for the application"
  type        = string
  default     = ""
}

variable "acm_certificate_arn" {
  description = "ACM certificate ARN for HTTPS"
  type        = string
  default     = ""
}

# MWAA Variables
variable "enable_mwaa" {
  description = "Enable Amazon MWAA deployment"
  type        = bool
  default     = false
}

variable "mwaa_environment_class" {
  description = "Environment class for MWAA"
  type        = string
  default     = "mw1.small"
}

variable "mwaa_max_workers" {
  description = "Maximum number of workers for MWAA"
  type        = number
  default     = 2
}

variable "mwaa_min_workers" {
  description = "Minimum number of workers for MWAA"
  type        = number
  default     = 1
}

variable "airflow_version" {
  description = "Apache Airflow version"
  type        = string
  default     = "2.7.2"
}

# Tags
variable "additional_tags" {
  description = "Additional tags to apply to resources"
  type        = map(string)
  default     = {}
}