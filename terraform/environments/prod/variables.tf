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
  default     = "prod"
}

# Network Configuration
variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.1.0.0/16" # Different from dev (10.0.0.0/16)
}

# RDS Aurora Variables (Production values)
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
  description = "Database password (use AWS Secrets Manager in production)"
  type        = string
  sensitive   = true
}

variable "database_min_capacity" {
  description = "Minimum Aurora Serverless v2 capacity"
  type        = number
  default     = 1.0 # Production recommendation
}

variable "database_max_capacity" {
  description = "Maximum Aurora Serverless v2 capacity"
  type        = number
  default     = 4.0 # Production recommendation
}

variable "database_backup_retention_period" {
  description = "Database backup retention period in days"
  type        = number
  default     = 30 # Production recommendation
}

variable "database_deletion_protection" {
  description = "Enable deletion protection for the database"
  type        = bool
  default     = true # Production recommendation
}

# ECS Variables (Production values)
variable "ecs_task_cpu" {
  description = "CPU units for ECS task"
  type        = string
  default     = "1024" # Production recommendation
}

variable "ecs_task_memory" {
  description = "Memory for ECS task in MB"
  type        = string
  default     = "2048" # Production recommendation
}

variable "ecs_desired_count" {
  description = "Desired number of ECS tasks"
  type        = number
  default     = 2 # High availability
}

variable "ecs_min_capacity" {
  description = "Minimum number of ECS tasks for auto scaling"
  type        = number
  default     = 2
}

variable "ecs_max_capacity" {
  description = "Maximum number of ECS tasks for auto scaling"
  type        = number
  default     = 10
}

# MWAA Variables (Production values)
variable "enable_mwaa" {
  description = "Enable Amazon MWAA deployment"
  type        = bool
  default     = true # Enable in production
}

variable "mwaa_environment_class" {
  description = "Environment class for MWAA"
  type        = string
  default     = "mw1.medium" # Production recommendation
}

variable "mwaa_max_workers" {
  description = "Maximum number of workers for MWAA"
  type        = number
  default     = 5 # Production recommendation
}

variable "mwaa_min_workers" {
  description = "Minimum number of workers for MWAA"
  type        = number
  default     = 1 # Production recommendation
}

# Logging and Monitoring
variable "log_retention_days" {
  description = "CloudWatch logs retention in days"
  type        = number
  default     = 30 # Production recommendation
}

variable "alb_log_retention_days" {
  description = "ALB access logs retention in days"
  type        = number
  default     = 90 # Compliance requirement
}

# Domain and Certificate
variable "domain_name" {
  description = "Domain name for the application"
  type        = string
  default     = "" # Set in terraform.tfvars
}

variable "certificate_arn" {
  description = "ACM certificate ARN for HTTPS"
  type        = string
  default     = "" # Set after creating certificate
}

# Auto Scaling
variable "cpu_target_value" {
  description = "Target CPU utilization for auto scaling"
  type        = number
  default     = 70
}

variable "memory_target_value" {
  description = "Target memory utilization for auto scaling"
  type        = number
  default     = 80
}

# Health Check
variable "health_check_grace_period" {
  description = "Health check grace period in seconds"
  type        = number
  default     = 60
}

variable "health_check_interval" {
  description = "Health check interval in seconds"
  type        = number
  default     = 30
}

variable "health_check_timeout" {
  description = "Health check timeout in seconds"
  type        = number
  default     = 10
}

variable "health_check_healthy_threshold" {
  description = "Number of consecutive health checks successes required"
  type        = number
  default     = 2
}

variable "health_check_unhealthy_threshold" {
  description = "Number of consecutive health check failures required"
  type        = number
  default     = 3
}

# Monitoring and Alerting
variable "enable_monitoring" {
  description = "Enable enhanced monitoring"
  type        = bool
  default     = true
}

variable "alarm_email" {
  description = "Email address for CloudWatch alarm notifications"
  type        = string
  default     = "" # Set in terraform.tfvars
}

# Cost Management
variable "enable_cost_allocation_tags" {
  description = "Enable cost allocation tags"
  type        = bool
  default     = true
}

# Security
variable "enable_waf" {
  description = "Enable AWS WAF for CloudFront"
  type        = bool
  default     = true
}

variable "allowed_ips" {
  description = "List of allowed IP addresses (leave empty for public access)"
  type        = list(string)
  default     = []
}

# Tags
variable "additional_tags" {
  description = "Additional tags to apply to all resources"
  type        = map(string)
  default = {
    Terraform  = "true"
    Owner      = "Engineering"
    CostCenter = "Engineering"
    Compliance = "PCI"
  }
}