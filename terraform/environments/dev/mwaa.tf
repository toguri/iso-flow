# Amazon MWAA (Managed Apache Airflow) Configuration

# MWAA Module
module "mwaa" {
  count  = var.enable_mwaa ? 1 : 0
  source = "../../modules/mwaa"

  project_name    = var.project_name
  environment     = var.environment
  vpc_id          = module.vpc.vpc_id
  private_subnets = module.vpc.private_subnet_ids

  dag_s3_bucket     = aws_s3_bucket.mwaa.id
  dag_s3_bucket_arn = aws_s3_bucket.mwaa.arn

  environment_class = var.mwaa_environment_class
  min_workers       = var.mwaa_min_workers
  max_workers       = var.mwaa_max_workers

  backend_api_endpoint = "http://${aws_lb.main.dns_name}"
  database_secret_arn  = module.rds.database_secret_arn

  tags = {
    Component = "Scheduler"
  }

  depends_on = [
    aws_s3_bucket_versioning.mwaa,
    aws_s3_object.dags,
    aws_s3_object.requirements
  ]
}

# Upload DAGs to S3
resource "aws_s3_object" "dags" {
  for_each = fileset("${path.module}/../../../airflow/dags", "*.py")

  bucket = aws_s3_bucket.mwaa.id
  key    = "dags/${each.value}"
  source = "${path.module}/../../../airflow/dags/${each.value}"
  etag   = filemd5("${path.module}/../../../airflow/dags/${each.value}")

  depends_on = [aws_s3_bucket_versioning.mwaa]
}

# Upload requirements.txt to S3
resource "aws_s3_object" "requirements" {
  bucket = aws_s3_bucket.mwaa.id
  key    = "requirements.txt"
  source = "${path.module}/../../../airflow/requirements.txt"
  etag   = filemd5("${path.module}/../../../airflow/requirements.txt")

  depends_on = [aws_s3_bucket_versioning.mwaa]
}

# Create Airflow variables in SSM Parameter Store
resource "aws_ssm_parameter" "airflow_backend_endpoint" {
  count = var.enable_mwaa ? 1 : 0

  name  = "/airflow/variables/backend_endpoint"
  type  = "String"
  value = "http://${aws_lb.main.dns_name}/graphql"

  tags = {
    Component = "Airflow"
  }
}

resource "aws_ssm_parameter" "airflow_scraping_interval" {
  count = var.enable_mwaa ? 1 : 0

  name  = "/airflow/variables/scraping_interval_minutes"
  type  = "String"
  value = "5"

  tags = {
    Component = "Airflow"
  }
}

# CloudWatch Log Groups for MWAA
resource "aws_cloudwatch_log_group" "mwaa_dag_processing" {
  count = var.enable_mwaa ? 1 : 0

  name              = "airflow-${var.project_name}-${var.environment}-DAGProcessing"
  retention_in_days = var.log_retention_days

  tags = {
    Component = "Airflow"
  }
}

resource "aws_cloudwatch_log_group" "mwaa_scheduler" {
  count = var.enable_mwaa ? 1 : 0

  name              = "airflow-${var.project_name}-${var.environment}-Scheduler"
  retention_in_days = var.log_retention_days

  tags = {
    Component = "Airflow"
  }
}

resource "aws_cloudwatch_log_group" "mwaa_task" {
  count = var.enable_mwaa ? 1 : 0

  name              = "airflow-${var.project_name}-${var.environment}-Task"
  retention_in_days = var.log_retention_days

  tags = {
    Component = "Airflow"
  }
}

resource "aws_cloudwatch_log_group" "mwaa_webserver" {
  count = var.enable_mwaa ? 1 : 0

  name              = "airflow-${var.project_name}-${var.environment}-WebServer"
  retention_in_days = var.log_retention_days

  tags = {
    Component = "Airflow"
  }
}

resource "aws_cloudwatch_log_group" "mwaa_worker" {
  count = var.enable_mwaa ? 1 : 0

  name              = "airflow-${var.project_name}-${var.environment}-Worker"
  retention_in_days = var.log_retention_days

  tags = {
    Component = "Airflow"
  }
}

# Outputs for MWAA
output "mwaa_environment_name" {
  description = "MWAA environment name"
  value       = var.enable_mwaa ? module.mwaa[0].environment_name : null
}

output "mwaa_webserver_url" {
  description = "MWAA webserver URL"
  value       = var.enable_mwaa ? module.mwaa[0].webserver_url : null
}

output "mwaa_execution_role_arn" {
  description = "MWAA execution role ARN"
  value       = var.enable_mwaa ? module.mwaa[0].execution_role_arn : null
}