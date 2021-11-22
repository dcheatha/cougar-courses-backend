use sea_orm::{Database, DatabaseConnection};

use crate::model::app::CoreResult;


pub async fn init() -> CoreResult<DatabaseConnection> {
  let database_url: String = std::env::var("DATABASE_URL").unwrap_or(String::from("postgres:///"));
  Ok(Database::connect(&database_url).await?)
}
