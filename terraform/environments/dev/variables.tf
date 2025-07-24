variable "aws_region" {
  description = "AWS region"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "environment" {
  description = "Environment name"
  type        = string
}

# VPC Variables
variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
}

variable "azs" {
  description = "Availability zones"
  type        = list(string)
}

# RDS Variables
variable "db_instance_class" {
  description = "RDS instance class"
  type        = string
  default     = "db.serverless"
}

variable "db_name" {
  description = "Database name"
  type        = string
}

variable "db_username" {
  description = "Database username"
  type        = string
}

variable "db_password" {
  description = "Database password"
  type        = string
  sensitive   = true
}

# ECS Variables
variable "ecs_task_cpu" {
  description = "ECS task CPU units"
  type        = number
}

variable "ecs_task_memory" {
  description = "ECS task memory (MB)"
  type        = number
}

# MWAA Variables
variable "mwaa_environment_class" {
  description = "MWAA environment class"
  type        = string
}

variable "mwaa_min_workers" {
  description = "MWAA minimum workers"
  type        = number
}

variable "mwaa_max_workers" {
  description = "MWAA maximum workers"
  type        = number
}

# Tags
variable "tags" {
  description = "Resource tags"
  type        = map(string)
}