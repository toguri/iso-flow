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
  value       = module.ecs.alb_dns_name
}

output "frontend_url" {
  description = "Frontend URL"
  value       = "http://${aws_s3_bucket_website_configuration.frontend.website_endpoint}"
}

output "mwaa_webserver_url" {
  description = "MWAA webserver URL"
  value       = var.enable_mwaa ? module.mwaa[0].webserver_url : null
}

output "ecr_repository_url" {
  description = "ECR repository URL for backend"
  value       = module.ecs.ecr_repository_url
}