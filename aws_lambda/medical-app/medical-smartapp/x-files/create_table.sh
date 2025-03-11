aws dynamodb create-table \
    --table-name SmartAppTokens \
    --attribute-definitions \
        '[
            {"AttributeName": "pk", "AttributeType": "S"},
            {"AttributeName": "client_id", "AttributeType": "S"}
        ]' \
    --key-schema \
        '[
            {"AttributeName": "pk", "KeyType": "HASH"}
        ]' \
    --provisioned-throughput \
        '{"ReadCapacityUnits": 5, "WriteCapacityUnits": 5}' \
    --global-secondary-indexes \
        '[
            {
                "IndexName": "client_id-index",
                "KeySchema": [
                    {"AttributeName": "client_id", "KeyType": "HASH"}
                ],
                "Projection": {
                    "ProjectionType": "INCLUDE",
                    "NonKeyAttributes": ["pk", "authorized", "session_timeout"]
                },
                "ProvisionedThroughput": {
                    "ReadCapacityUnits": 5,
                    "WriteCapacityUnits": 5
                }
            }
        ]' \
    --tags Key=Environment,Value=Production


###################################
# Only create the Index

aws dynamodb update-table \
    --table-name SmartAppTokens \
    --attribute-definitions '[
        {"AttributeName": "client_id", "AttributeType": "S"}
    ]' \
    --global-secondary-index-updates '[{
        "Create": {
            "IndexName": "client_id-index",
            "KeySchema": [{
                "AttributeName": "client_id",
                "KeyType": "HASH"
            }],
            "Projection": {
                "ProjectionType": "INCLUDE",
                "NonKeyAttributes": ["pk", "authorized", "session_timeout"]
            },
            "ProvisionedThroughput": {
                "ReadCapacityUnits": 5,
                "WriteCapacityUnits": 1
            }
        }
    }]'
