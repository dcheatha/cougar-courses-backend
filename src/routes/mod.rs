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
  use actix_web::{
    dev::{Body, ResponseBody},
    test, App,
  };

  trait BodyTest {
    fn as_str(&self) -> &str;
  }

  impl BodyTest for ResponseBody<Body> {
    fn as_str(&self) -> &str {
      match self {
        ResponseBody::Body(ref b) => match b {
          Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
          _ => panic!(),
        },
        ResponseBody::Other(ref b) => match b {
          Body::Bytes(ref by) => std::str::from_utf8(&by).unwrap(),
          _ => panic!(),
        },
      }
    }
  }

  #[tokio::test]
  async fn test_health_is_ok() {
    let mut server = test::init_service(App::new().configure(mount)).await;
    let request = test::TestRequest::get().uri("/health").to_request();
    let data: serde_json::Value = test::read_response_json(&mut server, request).await;

    assert_eq!(data, json!({ "health": "ok" }));
  }
}
