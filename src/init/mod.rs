use crate::model::app;

pub mod database;


pub async fn init() -> app::State {
  let database = database::init().await;

  app::State { database }
}