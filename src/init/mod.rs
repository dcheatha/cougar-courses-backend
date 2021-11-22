use crate::model::app;

pub mod actix;

mod database;
mod env_logger;


pub async fn init() -> app::CoreState {
  env_logger::init();

  let database = database::init().await;

  app::CoreState { database }
}