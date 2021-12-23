mod actix;
mod error;

use std::sync::Arc;

pub use actix::ActixState as ActixState;
pub use actix::ActixConfigVars as ActixConfigVars;

pub use error::CoreError as CoreError;
pub use error::CoreResult as CoreResult;

use super::graphql::GraphQLSchema;

#[derive(Clone)]
pub struct CoreState {
  pub database: Arc<sea_orm::DatabaseConnection>,
  pub graphql: GraphQLSchema,
}
