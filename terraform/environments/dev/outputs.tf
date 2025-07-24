# Outputs from all modules

output "vpc_id" {
  description = "VPC ID"
  value       = module.vpc.vpc_id
}

output "database_endpoint" {
  description = "RDS Aurora endpoint"
  value       = module.rds.cluster_endpoint
}

output "backend_api_endpoint" {
  description = "Backend API endpoint"
  value       = module.ecs.api_endpoint
}

output "frontend_url" {
  description = "Frontend URL"
  value       = module.s3.frontend_url
}

output "mwaa_webserver_url" {
  description = "MWAA webserver URL"
  value       = module.mwaa.webserver_url
}

output "ecr_repository_url" {
  description = "ECR repository URL for backend"
  value       = module.ecs.ecr_repository_url
}