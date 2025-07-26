# ECS Fargate for Backend API

module "ecs" {
  source = "../../modules/ecs"

  project_name = var.project_name
  environment  = var.environment
  
  vpc_id          = module.vpc.vpc_id
  private_subnets = module.vpc.private_subnet_ids
  public_subnets  = module.vpc.public_subnet_ids
  
  # Database connection
  database_url        = "postgresql://${var.database_username}:${var.database_password}@${module.rds.rds_endpoint}/${var.database_name}?sslmode=require"
  database_secret_arn = module.rds.db_secret_arn
  
  # Task configuration
  task_cpu    = var.ecs_task_cpu
  task_memory = var.ecs_task_memory
  
  tags = local.common_tags
}

# Output the ALB DNS name
output "backend_api_url" {
  description = "URL of the backend API"
  value       = "http://${module.ecs.alb_dns_name}"
}

# Output ECR repository URL for CI/CD
output "ecr_repository_url" {
  description = "ECR repository URL"
  value       = module.ecs.ecr_repository_url
}