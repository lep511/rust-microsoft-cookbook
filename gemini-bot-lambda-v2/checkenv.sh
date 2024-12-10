#!/bin/bash
# Array of required environment variables
required_vars=("MONGODB_PASSWORD" "MONGODB_USER_NAME" "GOOGLE_API_KEY")

# Function to check if variable exists
check_env_var() {
    local var_name=$1
    if [ -z "${!var_name}" ]; then
        echo "❌ $var_name is not set"
        return 1
    else
        echo "✅ $var_name is set"
        return 0
    fi
}

# Counter for missing variables
missing_vars=0

echo "Checking environment variables..."
echo "--------------------------------"

# Check each required variable
for var in "${required_vars[@]}"; do
    if ! check_env_var "$var"; then
        ((missing_vars++))
    fi
done

echo "--------------------------------"
if [ $missing_vars -gt 0 ]; then
    echo "Error: $missing_vars environment variable(s) missing"
    exit 1
else
    echo "All environment variables are properly set"
    exit 0
fi