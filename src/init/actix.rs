use actix_web::web;

use crate::model::app::{self, ActixConfigVars, ActixState};

pub async fn init() -> app::CoreResult<web::Data<app::ActixState>> {
  let core_state = super::init().await?;

  let config_vars = ActixConfigVars {
    listen_url: std::env::var("LISTEN_URL").unwrap_or(String::from("[::1]:8080")),
  };

  Ok(web::Data::new(ActixState {
    core_state,
    config_vars,
  }))
}
