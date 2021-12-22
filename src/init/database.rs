use sea_orm::{Database, DatabaseConnection};

use crate::model::app::CoreResult;

pub async fn init() -> CoreResult<DatabaseConnection> {
  let database_url: String = std::env::var("DATABASE_URL").unwrap_or_else(|_| String::from("postgres:///"));
  Ok(Database::connect(&database_url).await?)
}

#[cfg(test)]
pub mod tests {
  use super::*;

  use sea_orm::{
    DatabaseBackend, MockDatabase,
  };

  pub async fn init() -> CoreResult<DatabaseConnection> {
    Ok(MockDatabase::new(DatabaseBackend::Postgres).into_connection())
  }
}
