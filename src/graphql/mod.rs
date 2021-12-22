use async_graphql as gql;
use gql::Json;
use serde_json::json;

use crate::model::app;

// pub struct Query;

// #[gql::Object]
// impl Query {
//   async fn health(&self) -> Result<gql::Json<&str>, app::CoreError> {
//     Ok(json!({"health": "ok"}))
//   }
// }
