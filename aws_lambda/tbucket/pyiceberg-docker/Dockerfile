# Use Python 3.9 as the base image (compatible with PyIceberg)
FROM python:3.9-slim

# Set working directory
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements file
COPY requirements.txt .

# Install Python dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy the application code
COPY . .

# Set default AWS region if not provided
ENV AWS_DEFAULT_REGION=us-east-2

# Create a directory for AWS credentials
RUN mkdir -p /root/.aws

# Create an entrypoint script
RUN echo '#!/bin/bash\n\
# Generate AWS credentials file if environment variables are provided\n\
if [ ! -z "$AWS_ACCESS_KEY_ID" ] && [ ! -z "$AWS_SECRET_ACCESS_KEY" ]; then\n\
  echo "[default]" > /root/.aws/credentials\n\
  echo "aws_access_key_id = $AWS_ACCESS_KEY_ID" >> /root/.aws/credentials\n\
  echo "aws_secret_access_key = $AWS_SECRET_ACCESS_KEY" >> /root/.aws/credentials\n\
  echo "AWS credentials file created from environment variables"\n\
fi\n\
\n\
# Execute the command\n\
exec "$@"\n' > /app/entrypoint.sh

# Make entrypoint script executable
RUN chmod +x /app/entrypoint.sh

# Set entrypoint to our script
ENTRYPOINT ["/app/entrypoint.sh"]

# Default command to run the script
CMD ["python", "main.py", "--table", "customer"]