use std::sync::Arc;

use crate::model::app;

pub fn init(database: Arc<sea_orm::DatabaseConnection>) -> app::CoreDataLoader {
  app::CoreDataLoader::new(database)
}

#[cfg(test)]
pub mod tests {
  use super::*;

  pub fn init(database: Arc<sea_orm::DatabaseConnection>) -> app::CoreDataLoader {
    app::CoreDataLoader::new(database)
  }
}
