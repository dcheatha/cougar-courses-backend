use crate::model::app;

pub mod actix;

mod database;
mod env_logger;


pub async fn init() -> app::CoreResult<app::CoreState> {
  env_logger::init();

  let database = database::init().await?;

  Ok(app::CoreState { database })
}
