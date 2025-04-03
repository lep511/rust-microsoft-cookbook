# Configure the AWS Provider
provider "aws" {
  region = var.aws_region
  default_tags {
    tags = {
      env             = var.environment
      owner           = "Ops"
      applicationName = var.application_name
      awsApplication  = aws_servicecatalogappregistry_application.terraform_app.application_tag.awsApplication
      version         = var.version_app
      service         = var.application_name
      terraform       = "true"
    }    
  }
  
  # Make it faster by skipping something
  skip_metadata_api_check     = true
  skip_region_validation      = true
  skip_credentials_validation = true
}

# Create application using aliased 'application' provider
provider "aws" {
  alias = "application"
  region = var.aws_region
}

# Register new application
# An AWS Service Catalog AppRegistry Application is displayed in the AWS Console under "MyApplications".
resource "aws_servicecatalogappregistry_application" "terraform_app" {
  provider    = aws.application
  name        = var.application_name
  description = "Terraform application for SNS project"
}

# Create an SNS topic
resource "aws_sns_topic" "my_topic" {
  name = "my_topic"  # Name of the SNS topic
}

# Create an email subscription to the SNS topic
resource "aws_sns_topic_subscription" "email_subscription" {
  topic_arn = aws_sns_topic.my_topic.arn  # Reference the ARN of the created topic
  protocol  = "email"                     # Use the email protocol
  endpoint  = "estebanpbuday@yahoo.es"    # The email address to subscribe
}