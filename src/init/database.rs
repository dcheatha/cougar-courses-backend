use sea_orm::{Database, DatabaseConnection};


pub async fn init() -> DatabaseConnection {
  let database_url: String = std::env::var("DATABASE_URL").unwrap_or(String::from("postgres:///"));
  Database::connect(&database_url).await.unwrap()
}
