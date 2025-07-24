# VPC Module - Not needed as we're using terraform-aws-modules/vpc
# This file is kept for reference only

# The actual VPC is created in environments/dev/main.tf using:
# module "vpc" {
#   source = "terraform-aws-modules/vpc/aws"
#   ...
# }