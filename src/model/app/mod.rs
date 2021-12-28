mod actix;
mod dataloader;
mod error;

use std::sync::Arc;

pub use actix::ActixConfigVars;
pub use actix::ActixState;

pub use error::CoreError;
pub use error::CoreResult;

use crate::graphql::GraphQLSchema;

pub use self::dataloader::CoreDataLoader;

pub struct CoreState {
  pub database: Arc<sea_orm::DatabaseConnection>,
  pub graphql: GraphQLSchema,
  pub dataloader: CoreDataLoader,
}
