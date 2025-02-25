aws dynamodb create-table \
    --table-name SmartAppTokens \
    --attribute-definitions \
        AttributeName=session_state,AttributeType=S \
    --key-schema \
        AttributeName=session_state,KeyType=HASH \
    --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5 \
    --tags Key=Environment,Value=Production