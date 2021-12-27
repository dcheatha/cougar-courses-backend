use actix_web::{http, web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use serde_json::json;

use crate::model::app;

pub fn mount(router: &mut web::ServiceConfig) {
  router
    .route("/health", web::get().to(health))
    .route("/graphql", web::post().to(graphql))
    .route("/graphql", web::get().to(playground));
}

async fn health() -> HttpResponse {
  HttpResponse::build(http::StatusCode::OK).json(json!({ "health": "ok" }))
}

async fn graphql(state: web::Data<app::CoreState>, request: GraphQLRequest) -> GraphQLResponse {
  let graphql = state.graphql.clone();
  let request = request.into_inner().data(state.into_inner());
  graphql.execute(request).await.into()
}

async fn playground() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(playground_source(
      GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_http::Request;
  use actix_web::{dev::Service, test, App};

  async fn setup_server(
  ) -> impl Service<Request, Error = actix_web::Error, Response = actix_web::dev::ServiceResponse>
  {
    test::init_service(App::new().configure(mount)).await
  }

  #[tokio::test]
  async fn test_health_is_ok() {
    let server = setup_server().await;
    let request = test::TestRequest::get().uri("/health").to_request();
    let data: serde_json::Value = test::call_and_read_body_json(&server, request).await;

    assert_eq!(data, json!({ "health": "ok" }));
  }
}
