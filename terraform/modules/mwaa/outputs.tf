output "environment_name" {
  description = "MWAA environment name"
  value       = aws_mwaa_environment.main.name
}

output "environment_arn" {
  description = "MWAA environment ARN"
  value       = aws_mwaa_environment.main.arn
}

output "webserver_url" {
  description = "MWAA webserver URL"
  value       = aws_mwaa_environment.main.webserver_url
}

output "execution_role_arn" {
  description = "MWAA execution role ARN"
  value       = aws_iam_role.mwaa_execution.arn
}

output "results_bucket_name" {
  description = "S3 bucket for MWAA results"
  value       = aws_s3_bucket.mwaa_results.id
}

output "security_group_id" {
  description = "MWAA security group ID"
  value       = aws_security_group.mwaa.id
}