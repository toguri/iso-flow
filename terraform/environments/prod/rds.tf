# RDS Aurora Serverless v2 Module for Production
module "rds" {
  source = "../../modules/rds"

  project_name     = var.project_name
  environment      = var.environment
  vpc_id           = module.vpc.vpc_id
  database_subnets = module.vpc.database_subnet_ids

  # Database configuration
  db_name     = var.database_name
  db_username = var.database_username
  db_password = var.database_password

  tags = merge(
    var.additional_tags,
    {
      Component = "Database"
      Backup    = "Daily"
      Critical  = "Yes"
    }
  )
}

# Security Group Rule to allow ECS tasks to connect to RDS
resource "aws_security_group_rule" "ecs_to_rds" {
  type                     = "ingress"
  from_port                = 5432
  to_port                  = 5432
  protocol                 = "tcp"
  source_security_group_id = module.ecs.ecs_service_security_group_id
  security_group_id        = module.rds.security_group_id
  description              = "Allow ECS tasks to connect to RDS"
}

# MWAA security group rule will be added when MWAA is enabled

# Store database connection URL in Secrets Manager (not SSM for production)
resource "aws_secretsmanager_secret" "database_url" {
  name                    = "${var.project_name}/${var.environment}/database/connection-string"
  description             = "Database connection string for ${var.project_name} ${var.environment}"
  recovery_window_in_days = 30

  tags = merge(
    var.additional_tags,
    {
      Component = "Database"
    }
  )
}

resource "aws_secretsmanager_secret_version" "database_url" {
  secret_id     = aws_secretsmanager_secret.database_url.id
  secret_string = module.rds.database_url
}

# Outputs for RDS
output "rds_cluster_endpoint" {
  description = "Aurora cluster endpoint"
  value       = module.rds.cluster_endpoint
  sensitive   = true
}

output "rds_cluster_reader_endpoint" {
  description = "Aurora cluster reader endpoint"
  value       = module.rds.cluster_reader_endpoint
  sensitive   = true
}

output "rds_security_group_id" {
  description = "Security group ID for Aurora"
  value       = module.rds.security_group_id
}

output "database_secret_arn" {
  description = "ARN of the Secrets Manager secret containing database credentials"
  value       = module.rds.database_secret_arn
}

output "database_url_secret_arn" {
  description = "ARN of the Secrets Manager secret containing database connection string"
  value       = aws_secretsmanager_secret.database_url.arn
}