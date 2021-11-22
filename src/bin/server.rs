use actix_web::{App, HttpServer};
use tokio::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
  let listen_url: String = std::env::var("LISTEN_URL").unwrap_or(String::from("[::1]:8080"));

  HttpServer::new(move || App::new())
    .bind(listen_url)?
    .run()
    .await
}
