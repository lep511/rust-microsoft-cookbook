### AWS Service Catalog AppRegistry.

Run the following command in your terminal to creates a new application named "terraform-app" in AWS Service Catalog AppRegistry.

``` bash
aws servicecatalog-appregistry create-application --name terraform-app
```

#### Why Use This Command?
Creating an application in AppRegistry is the first step to organizing your AWS resources. Once "terraform-app" exists, you can:

* **Link Resources**: Associate things like CloudFormation stacks or S3 buckets to it using associate-resource.
* **Add Metadata**: Tag it with key-value pairs (e.g., environment=production) using tag-resource.
* **Monitor and Manage**: Use it as a reference point for tracking costs, compliance, or architecture.

### Update import block

Find the `imports.tf` file and replace the id key with the real ID.

``` json
import {
  to = aws_servicecatalogappregistry_application.terraform_app
  id = "app-xyz789"  # Replace with the real APP_ID
}
```

After save the application ID and run the following command in your terminal to initialize the Terraform working directory, downloading the necessary provider plugins:

``` bash
terraform init
```

* Execute the import command:

``` bash
terraform aws_servicecatalogappregistry_application.terraform_app {APP_ID}
```

### Plan the Deployment

Preview the changes Terraform will make:

``` bash
terraform plan
```

### Apply the Configuration

Deploy the resources to AWS:

``` bash
terraform apply
```

Type yes when prompted to confirm.