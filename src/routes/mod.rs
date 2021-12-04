use actix_web::{http, web, HttpResponse};
use serde_json::json;

pub fn mount(router: &mut web::ServiceConfig) {
  router.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
  HttpResponse::build(http::StatusCode::OK).json(json!({ "health": "ok" }))
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_http::Request;
  use actix_web::{
    dev::{Service, ServiceResponse},
    test, App,
  };

  async fn setup_server(
  ) -> impl Service<Request = Request, Response = ServiceResponse, Error = actix_web::Error> {
    test::init_service(App::new().configure(mount)).await
  }

  #[tokio::test]
  async fn test_health_is_ok() {
    let mut server = setup_server().await;
    let request = test::TestRequest::get().uri("/health").to_request();
    let data: serde_json::Value = test::read_response_json(&mut server, request).await;

    assert_eq!(data, json!({ "health": "ok" }));
  }
}
