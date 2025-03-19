#!/bin/bash

# Prompt for required TABLE_BUCKET_ARN with no default
read -p "Enter TABLE_BUCKET_ARN (Required): " table_bucket_arn
if [ -z "$table_bucket_arn" ]; then
  echo "Error: TABLE_BUCKET_ARN is required"
  exit 1
fi

# Prompt for optional variables with defaults
read -p "Enter TEMPLATE_PATH (default: templates/table_template.yaml): " template_path
template_path=${template_path:-templates/table_template.yaml}

read -p "Enter ATHENA_BUCKET (default: none): " athena_bucket
# athena_bucket will be empty string if not provided

read -p "Enter XAI_API_KEY (default: none): " xai_api_key
# xai_api_key will be empty string if not provided

# Export all variables
export TABLE_BUCKET_ARN="$table_bucket_arn"
export TEMPLATE_PATH="$template_path"
export ATHENA_BUCKET="$athena_bucket"
export XAI_API_KEY="$xai_api_key"

echo " "
echo "Variables set successfully!"
echo " "
echo " "