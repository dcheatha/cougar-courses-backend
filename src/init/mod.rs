use crate::model::app;

pub mod actix;

mod database;
mod env_logger;
mod graphql;

pub async fn init() -> app::CoreResult<app::CoreState> {
  env_logger::init();

  let database = database::init().await?;
  let graphql = graphql::init();

  Ok(app::CoreState { database, graphql })
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub async fn init() -> app::CoreResult<app::CoreState> {
    env_logger::tests::init();

    let database = database::tests::init().await?;
    let graphql = graphql::init();

    Ok(app::CoreState { database, graphql })
  }
}
