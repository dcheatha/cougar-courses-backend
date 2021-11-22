use actix_web::web;

use crate::model::app;


pub async fn init() -> app::ActixState {
  let core_state = super::init().await;

  web::Data::new(core_state)
}