mod grades;

use std::sync::Arc;

use grades::GradesDataLoader;

pub struct CoreDataLoader {
  pub grades: GradesDataLoader,
}

impl CoreDataLoader {
  pub fn new(database: Arc<sea_orm::DatabaseConnection>) -> CoreDataLoader {
    CoreDataLoader {
      grades: GradesDataLoader { database },
    }
  }
}
