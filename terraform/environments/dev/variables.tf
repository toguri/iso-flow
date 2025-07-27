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

# Logging
variable "log_retention_days" {
  description = "CloudWatch logs retention in days"
  type        = number
  default     = 7
}

# Domain and Certificate (optional)
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

# Tags
