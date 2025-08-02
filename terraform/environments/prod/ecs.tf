# ECS Fargate for Backend API - Production Configuration (Simplified)

module "ecs" {
  source = "../../modules/ecs"

  project_name = var.project_name
  environment  = var.environment

  vpc_id          = module.vpc.vpc_id
  private_subnets = module.vpc.private_subnet_ids
  public_subnets  = module.vpc.public_subnet_ids

  # Database connection
  database_url        = "postgresql://${var.database_username}:${var.database_password}@${module.rds.cluster_endpoint}/${var.database_name}?sslmode=require"
  database_secret_arn = module.rds.database_secret_arn

  # Task configuration for production
  task_cpu    = var.ecs_task_cpu
  task_memory = var.ecs_task_memory

  tags = merge(
    var.additional_tags,
    {
      Component = "Backend"
      Service   = "API"
    }
  )
}

# Output the ALB DNS name
output "backend_api_url" {
  description = "URL of the backend API"
  value       = "http://${module.ecs.alb_dns_name}"
}

output "ecs_cluster_name" {
  description = "Name of the ECS cluster"
  value       = module.ecs.cluster_name
}

output "ecs_service_name" {
  description = "Name of the ECS service"
  value       = module.ecs.service_name
}

output "alb_dns_name" {
  description = "DNS name of the load balancer"
  value       = module.ecs.alb_dns_name
}