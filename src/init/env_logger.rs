pub fn init() {
  env_logger::builder()
    .filter_level(log::LevelFilter::Info)
    .init()
}

#[cfg(test)]
pub mod tests {
  pub fn init() {
    env_logger::builder()
      .filter_level(log::LevelFilter::Trace)
      .is_test(true)
      .init()
  }
}
