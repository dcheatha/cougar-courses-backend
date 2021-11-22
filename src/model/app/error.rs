use log::error;
use tokio::io;

pub struct CoreError {
  source: String,
  message: String,
}

impl CoreError {
  pub fn new(source: String, message: String) -> CoreError {
    let error = CoreError {
      source, message
    };

    error!("CoreError: {}", error);

    error
  }
}

impl std::fmt::Display for CoreError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}: {}", self.source, self.message)
  }
}

pub type CoreResult<T> = Result<T, CoreError>;

impl From<anyhow::Error> for CoreError {
  fn from(error: anyhow::Error) -> CoreError {
    CoreError::new("Anyhow".to_string(), format!("{}", error))
  }
}

impl From<sea_orm::DbErr> for CoreError {
  fn from(error: sea_orm::DbErr) -> CoreError {
    CoreError::new("SeaOrm".to_string(), format!("{}", error))
  }
}


impl From<CoreError> for io::Error {
  fn from(error: CoreError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{}", error))
  }
}

