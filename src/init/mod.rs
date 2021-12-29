use crate::model::app;

pub mod actix;

mod database;
mod dataloader;
mod env_logger;
mod graphql;

pub async fn init() -> app::CoreResult<app::CoreState> {
  env_logger::init();

  let database = database::init().await?;
  let dataloader = dataloader::init(database.clone());
  let graphql = graphql::init();

  Ok(app::CoreState {
    database,
    dataloader,
    graphql,
  })
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub async fn init() -> app::CoreResult<app::CoreState> {
    env_logger::tests::init();

    let database = database::tests::init().await?;
    let dataloader = dataloader::tests::init(database.clone());
    let graphql = graphql::init();

    Ok(app::CoreState {
      database,
      dataloader,
      graphql,
    })
  }
}
