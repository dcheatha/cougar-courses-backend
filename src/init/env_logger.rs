pub fn init() {
  env_logger::builder()
    .filter_level(log::LevelFilter::Debug)
    .is_test(true)
    .init()
}