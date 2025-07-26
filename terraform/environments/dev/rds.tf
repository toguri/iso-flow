# RDS Aurora Serverless v2 Module
module "rds" {
  source = "../../modules/rds"
  
  project_name     = var.project_name
  environment      = var.environment
  vpc_id           = module.vpc.vpc_id
  database_subnets = module.vpc.database_subnet_ids
  
  db_name     = var.database_name
  db_username = var.database_username
  db_password = var.database_password
  
  tags = {
    Component = "Database"
  }
}

# Security Group Rule to allow ECS tasks to connect to RDS
resource "aws_security_group_rule" "ecs_to_rds" {
  type                     = "ingress"
  from_port                = 5432
  to_port                  = 5432
  protocol                 = "tcp"
  source_security_group_id = module.ecs.ecs_service_security_group_id
  security_group_id        = module.rds.security_group_id
}

# Security Group Rule to allow MWAA to connect to RDS
resource "aws_security_group_rule" "mwaa_to_rds" {
  count = var.enable_mwaa ? 1 : 0
  
  type                     = "ingress"
  from_port                = 5432
  to_port                  = 5432
  protocol                 = "tcp"
  source_security_group_id = module.mwaa[0].security_group_id
  security_group_id        = module.rds.security_group_id
}

# Store database connection URL in SSM Parameter Store
resource "aws_ssm_parameter" "database_url" {
  name  = "/${var.project_name}/${var.environment}/database_url"
  type  = "SecureString"
  value = module.rds.database_url
  
  tags = {
    Component = "Database"
  }
}

# Outputs for RDS
output "rds_cluster_endpoint" {
  description = "Aurora cluster endpoint"
  value       = module.rds.cluster_endpoint
}

output "rds_cluster_reader_endpoint" {
  description = "Aurora cluster reader endpoint"
  value       = module.rds.cluster_reader_endpoint
}

output "rds_security_group_id" {
  description = "Security group ID for Aurora"
  value       = module.rds.security_group_id
}

output "database_secret_arn" {
  description = "ARN of the Secrets Manager secret containing database credentials"
  value       = module.rds.database_secret_arn
}