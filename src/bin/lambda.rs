use lambda_runtime as lambda;
use serde_json::json;
use lib::{init, model::app};

#[tokio::main]
async fn main() -> Result<(), lambda::Error> {
  lambda::run(lambda::handler_fn(startup_main)).await?;
  Ok(())
}

async fn startup_main(
  value: serde_json::Value,
  context: lambda::Context,
) -> Result<serde_json::Value, lambda::Error> {
  let core_state = init::init().await?;
  run_main(value, context, core_state).await
}

async fn run_main(
  _: serde_json::Value,
  _: lambda::Context,
  _: app::CoreState,
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
    let test_state = super::init::tests::init().unwrap();

    let result = super::run_main(request, context, test_state);
    let result = result.await.ok().unwrap();
    assert_eq!(result.get("message").unwrap(), "Hello!");
  }
}
