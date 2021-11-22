# Cougar Courses Backend

## Deployment

### Managed server
```sh
# Build the actix web server
cargo build --release --bin server
# Manage it yourself
```

### AWS Lambda
```sh
set $lambdaFunction cougarCoursesGraphQL

# Create the aws role for lambda
aws iam create-role --role-name cougar-courses --assume-role-policy-document '{"Statement": [{ "Effect": "Allow", "Principal": {"Service": "lambda.amazonaws.com"}, "Action": "sts:AssumeRole"}]}'
set $lambdaRole arn:aws:iam:xxxxx:role/cougar-courses

# Cross-compile the lambda binary for x86_64-unknown-linux-musl
cross build --target x86_64-unknown-linux-musl --release --bin lambda

# Rename the binary to bootstrap and toss it into a zip file
cp target/x86_64-unknown-linux-musl/release/lambda ./bootstrap
zip -r9 -j bootstrap.zip bootstrap

# Create function on lambda
aws lambda create-function --function-name $lambdaFunction --zip-file fileb://./bootstrap.zip --role $lambdaRole --
runtime provided.al2 --handler none

# Update function on lambda
aws lambda update-function-code --function-name $lambdaFunction --zip-file fileb://./bootstrap.zip
```