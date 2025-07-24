output "frontend_bucket_name" {
  description = "Frontend S3 bucket name"
  value       = aws_s3_bucket.frontend.id
}

output "frontend_bucket_arn" {
  description = "Frontend S3 bucket ARN"
  value       = aws_s3_bucket.frontend.arn
}

output "frontend_bucket_domain_name" {
  description = "Frontend S3 bucket domain name"
  value       = aws_s3_bucket.frontend.bucket_domain_name
}

output "mwaa_dag_bucket" {
  description = "MWAA DAG S3 bucket name"
  value       = aws_s3_bucket.mwaa_dags.id
}

output "mwaa_dag_bucket_arn" {
  description = "MWAA DAG S3 bucket ARN"
  value       = aws_s3_bucket.mwaa_dags.arn
}

output "cloudfront_distribution_id" {
  description = "CloudFront distribution ID"
  value       = aws_cloudfront_distribution.frontend.id
}

output "cloudfront_domain_name" {
  description = "CloudFront domain name"
  value       = aws_cloudfront_distribution.frontend.domain_name
}

output "frontend_url" {
  description = "Frontend URL"
  value       = "https://${aws_cloudfront_distribution.frontend.domain_name}"
}