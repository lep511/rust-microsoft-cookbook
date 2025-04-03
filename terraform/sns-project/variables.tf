# The region is specified in  GitHub - settings/variables/actions
variable "application_name" {
  type        = string
  description = "The name of the application"
  default     = "SNS-Project"
}

variable "environment" {
  type        = string
  description = "The name of the environment"
  default     = "test"
}

variable "version_app" {
  type        = string
  description = "The version of the application"
  default     = "0.1.0"
}

variable "aws_region" {
  type        = string
  description = "The AWS region to deploy the application"
  default     = "us-west-2"
}