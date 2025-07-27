variable "project_name" {
  description = "Project name"
  type        = string
}

variable "environment" {
  description = "Environment name"
  type        = string
}

variable "vpc_id" {
  description = "VPC ID"
  type        = string
}

variable "private_subnets" {
  description = "List of private subnet IDs (minimum 2)"
  type        = list(string)
}


variable "dag_s3_bucket_arn" {
  description = "S3 bucket ARN for DAGs"
  type        = string
}

variable "environment_class" {
  description = "MWAA environment class"
  type        = string
  default     = "mw1.small"
}

variable "min_workers" {
  description = "Minimum number of workers"
  type        = number
  default     = 1
}

variable "max_workers" {
  description = "Maximum number of workers"
  type        = number
  default     = 2
}

variable "backend_api_endpoint" {
  description = "Backend API endpoint URL"
  type        = string
}

variable "database_secret_arn" {
  description = "ARN of the database secret"
  type        = string
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}