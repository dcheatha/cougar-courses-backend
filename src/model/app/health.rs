use serde::{Deserialize, Serialize};

use super::CoreState;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum CoreHealthStatus {
  Ok,
  Error
}

#[derive(Deserialize, Serialize)]
pub struct CoreHealth {
  pub status: CoreHealthStatus,
}

impl CoreHealth {
  pub fn ok() -> CoreHealth {
    CoreHealth {
      status: CoreHealthStatus::Ok,
    }
  }

  pub fn from(_core_state: &CoreState) -> CoreHealth {
    CoreHealth::ok()
  }
}