mod actix;
mod error;

pub use actix::ActixConfigVars;
pub use actix::ActixState;

pub use error::CoreError;
pub use error::CoreResult;

use crate::graphql::GraphQLSchema;

pub struct CoreState {
  pub database: sea_orm::DatabaseConnection,
  pub graphql: GraphQLSchema,
}
