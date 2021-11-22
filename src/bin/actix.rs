use actix_web::{App, HttpServer};
use tokio::io;

use lib::{init, routes};

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
  let actix_state = init::actix::init().await?;

  HttpServer::new(move || {
    App::new()
      .configure(routes::mount)
      .app_data(actix_state.clone())
  })
  .bind("[::1]:8080")?
  .run()
  .await
}
