# PyIceberg AWS Integration Docker Solution

This project provides a Docker-based solution for running the PyIceberg AWS integration code. It helps you interact with Apache Iceberg tables stored in AWS S3 buckets via AWS Glue.

## Prerequisites

- Docker installed on your system
- AWS account with appropriate permissions
- AWS credentials configured

## File Structure

```
pyiceberg-aws/
├── Dockerfile
├── docker-compose.yml
├── requirements.txt
├── main.py (your provided Python code)
├── .env (optional, for storing environment variables)
└── README.md
```

## Setup Instructions

1. Save your Python code as `main.py` in the project directory.

2. Copy the Dockerfile, docker-compose.yml, and requirements.txt from this package.

3. Set up your AWS credentials using one of these methods:

   **Option A: Environment Variables** (recommended for CI/CD pipelines)
   
   Create a `.env` file in the project directory:
   ```
   AWS_ACCESS_KEY_ID=your_access_key
   AWS_SECRET_ACCESS_KEY=your_secret_key
   AWS_REGION=us-east-2
   TABLE_NAME=your_table_name
   CATALOG_NAME=S3TablesCatalog
   S3_BUCKET=pyiceberg-blog-bucket
   DATABASE_NAME=myblognamespace
   ```

   **Option B: Mount AWS Credentials** (recommended for local development)
   
   Ensure your AWS credentials are configured in `~/.aws/credentials` and they'll be mounted into the container.

## Running the Application

### Using Docker Compose (Recommended)

```bash
# Build and run the container
docker-compose up --build

# Specify different table/catalog/bucket/database
TABLE_NAME=mycustomers CATALOG_NAME=MyCustomCatalog S3_BUCKET=my-custom-bucket DATABASE_NAME=mydb docker-compose up
```

### Using Docker Directly

```bash
# Build the Docker image
docker build -t pyiceberg-aws .

# Run with environment variables
docker run -e AWS_ACCESS_KEY_ID=your_access_key -e AWS_SECRET_ACCESS_KEY=your_secret_key -e AWS_DEFAULT_REGION=us-east-2 pyiceberg-aws python main.py --table customer

# Run with mounted credentials
docker run -v ${HOME}/.aws:/root/.aws:ro pyiceberg-aws python main.py --table customer --catalog S3TablesCatalog --s3tablebucket pyiceberg-blog-bucket --database myblognamespace
```

## Customization

You can customize the Docker execution by:

1. Editing the `docker-compose.yml` file to change default values
2. Creating a `.env` file with your specific configuration
3. Passing environment variables when running `docker-compose up`
4. Modifying the `Dockerfile` to include additional dependencies

## Troubleshooting

- **AWS Credentials Issues**: Ensure your AWS credentials have the necessary permissions for S3 and Glue.
- **Region Mismatch**: Verify that you're using the correct AWS region where your S3 bucket is located.
- **Dependency Issues**: Check the logs for any Python dependency issues. You may need to update the `requirements.txt` file.

## Security Notes

- Never commit your AWS credentials to version control
- Use environment variables or mounted credentials instead
- Consider using AWS IAM roles for production environments