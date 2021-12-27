use actix_web::{middleware::Logger, web, App, HttpServer};
use tokio::io;

use lib::{init, routes};

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
  let actix_state = init::actix::init().await?;
  let listen_url = &actix_state.config_vars.listen_url.clone();
  let core_state = web::Data::new(init::init().await?);

  HttpServer::new(move || {
    App::new()
      .configure(routes::mount)
      .app_data(core_state.clone())
      .wrap(Logger::new("%{r}a %r %s %bb %Dms"))
  })
  .bind(listen_url)?
  .run()
  .await
}
