mod actix;
mod error;

pub use actix::ActixState as ActixState;
pub use actix::ActixConfigVars as ActixConfigVars;

pub use error::CoreError as CoreError;
pub use error::CoreResult as CoreResult;

use crate::graphql::GraphQLSchema;

pub struct CoreState {
  pub database: sea_orm::DatabaseConnection,
  pub graphql: GraphQLSchema,
}
