use lambda_runtime as lambda;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), lambda::Error> {
  lambda::run(lambda::handler_fn(hello)).await?;
  Ok(())
}

async fn hello(
  _: serde_json::Value,
  _: lambda::Context,
) -> Result<serde_json::Value, lambda::Error> {
  Ok(json!({ "message": "Hello!" }))
}
