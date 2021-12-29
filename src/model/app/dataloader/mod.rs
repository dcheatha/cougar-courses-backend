mod grades;

use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use grades::GradesDataLoader;

pub struct CoreDataLoader {
  pub grades: DataLoader<GradesDataLoader>,
}

impl CoreDataLoader {
  pub fn new(database: Arc<sea_orm::DatabaseConnection>) -> CoreDataLoader {
    CoreDataLoader {
      grades: DataLoader::new(GradesDataLoader { database }, tokio::spawn),
    }
  }
}
