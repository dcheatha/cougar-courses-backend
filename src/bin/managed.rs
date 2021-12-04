use actix_web::{App, HttpServer};
use tokio::io;

use lib::{init, routes};

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
  let actix_state = init::actix::init().await?;
  let listen_url = &actix_state.config_vars.listen_url.clone();

  HttpServer::new(move || {
    App::new()
      .configure(routes::mount)
      .app_data(actix_state.clone())
  })
  .bind(listen_url)?
  .run()
  .await
}
