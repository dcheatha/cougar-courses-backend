use actix_web::http::header::Accept;

pub struct ActixState {
  pub core_state: super::CoreState,
  pub config_vars: ActixConfigVars,
}

pub struct ActixConfigVars {
  pub listen_url: String,
}
