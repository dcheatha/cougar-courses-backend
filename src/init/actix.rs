use actix_web::web;

use crate::model::app::{self, ActixConfigVars, ActixState};

pub async fn init() -> app::CoreResult<web::Data<app::ActixState>> {
  let config_vars = ActixConfigVars {
    listen_url: std::env::var("LISTEN_URL").unwrap_or_else(|_| String::from("localhost:8080")),
  };

  Ok(web::Data::new(ActixState { config_vars }))
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use actix_http::Request;
  use actix_web::{dev::Service, test, App};

  use crate::routes;

  pub async fn init() -> app::CoreResult<web::Data<app::ActixState>> {
    let config_vars = ActixConfigVars {
      listen_url: std::env::var("LISTEN_URL").unwrap_or_else(|_| String::from("localhost:8080")),
    };

    Ok(web::Data::new(ActixState { config_vars }))
  }

  pub async fn init_test_server() -> impl Service<Request> {
    let test_state = tests::init().await.unwrap();

    test::init_service(
      App::new()
        .configure(routes::mount)
        .app_data(test_state.clone()),
    )
    .await
  }
}
