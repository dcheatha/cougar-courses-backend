use actix_web::{App, HttpServer};
use tokio::io;

use lib::init;

#[actix_web::main]
async fn main() -> io::Result<()> {
  let listen_url: String = std::env::var("LISTEN_URL").unwrap_or(String::from("[::1]:8080"));

  let actix_state = init::actix::init().await;

  HttpServer::new(move || {
    App::new()
      // .configure(routes::mount)
      .app_data(actix_state.clone())
  })
  .bind(listen_url)?
  .run()
  .await
}
