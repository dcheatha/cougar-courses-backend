use actix_web::web;


pub struct CoreState {
  pub database: sea_orm::DatabaseConnection,
}

pub type ActixState = web::Data<CoreState>;
