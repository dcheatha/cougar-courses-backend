use actix_web::web;

use crate::model::app::{self, ActixConfigVars, ActixState};

pub async fn init() -> app::CoreResult<web::Data<app::ActixState>> {
  let core_state = super::init().await?;

  let config_vars = ActixConfigVars {
    listen_url: std::env::var("LISTEN_URL").unwrap_or_else(|_| String::from("localhost:8080")),
  };

  Ok(web::Data::new(ActixState {
    core_state,
    config_vars,
  }))
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use actix_http::Request;
  use actix_web::{
    dev::{Service, ServiceResponse},
    test, App,
  };

  use crate::{init, routes};

  pub async fn init() -> app::CoreResult<web::Data<app::ActixState>> {
    let core_state = init::tests::init().await?;

    let config_vars = ActixConfigVars {
      listen_url: std::env::var("LISTEN_URL").unwrap_or_else(|_| String::from("localhost:8080")),
    };

    Ok(web::Data::new(ActixState {
      core_state,
      config_vars,
    }))
  }

  pub async fn init_test_server(
  ) -> impl Service<Request = Request, Response = ServiceResponse, Error = actix_web::Error> {
    let test_state = tests::init().await.unwrap();

    test::init_service(
      App::new()
        .configure(routes::mount)
        .app_data(test_state.clone()),
    )
    .await
  }
}
