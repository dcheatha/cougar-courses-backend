# Cougar Courses Backend
This is the backend that powers https://cougarcourses.app. 

If you're a WSU student, faculty, or alumni, feel free to consume this api! Attribution is appreciated.

## First-time setup

### Database
Create the database and import all data from the PDF:
```sh
export DATABASE_URL=postgresql://some-sql-url
sqlx database create
sqlx mig run
python3 import/convert_pdf_to_csv_data.py
cargo run --release --bin import_csv_data
```

## Deployment

### Pre-deploy steps

#### Verifying unit tests
* Run `cargo test`

#### Environment variables
Most (really, all) uses of env vars should be found in the `src/init` directory.
* `LISTEN_URL`: Http listen url if you're running a managed server (Defaults to `localhost:8080`)
* `DATABASE_URL`: Postgres database url (Defaults to `postgres://`, sometimes stringent environments may require `postgresql://` instead)

#### SQL Migrations
* To install the SQLx migrator: `cargo install sqlx-cli --no-default-features --features postgres`
* To run migrations: `sqlx mig run`
* To add a new migration: `sqlx mig add name_of_migration`

### Deployment targets
#### Managed server
```sh
# Build the actix web server
cargo build --release --bin managed
# Run & manage it yourself
```

#### AWS Lambda
```sh
set $lambdaFunction cougarCoursesGraphQL

# Create the aws role for lambda & allow it to use lambda services
aws iam create-role --role-name cougar-courses --assume-role-policy-document '{"Statement": [{ "Effect": "Allow", "Principal": {"Service": "lambda.amazonaws.com"}, "Action": "sts:AssumeRole"}]}'
set $lambdaRole arn:aws:iam:xxxxx:role/cougar-courses

# Cross-compile the lambda binary for x86_64-unknown-linux-musl (uses docker) (AWS lambda requirement)
cross build --target x86_64-unknown-linux-musl --release --bin lambda

# Rename the binary to bootstrap and toss it into a zip file (AWS lambda looks for executables with this name)
cp target/x86_64-unknown-linux-musl/release/lambda ./bootstrap
zip -r9 -j bootstrap.zip bootstrap

# Create function on lambda if it doesn't exist
aws lambda create-function --function-name $lambdaFunction --zip-file fileb://./bootstrap.zip --role $lambdaRole --
runtime provided.al2 --handler none

# Update function on lambda if it does exist
aws lambda update-function-code --function-name $lambdaFunction --zip-file fileb://./bootstrap.zip
```