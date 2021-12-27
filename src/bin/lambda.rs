use lambda_runtime as lambda;
use lib::{init, model::app};
use serde_json::json;

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
