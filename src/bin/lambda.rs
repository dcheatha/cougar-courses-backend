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

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_hello_handler() {
    let context = lambda::Context::default();
    let request = json!({});

    let result = super::hello(request, context);
    let result = result.await.ok().unwrap();
    assert_eq!(result.get("message").unwrap(), "Hello!");
  }
}
