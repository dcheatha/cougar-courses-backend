use async_graphql as gql;
use statrs::statistics::{Data, Distribution, OrderStatistics, Statistics};

pub struct DiscreteStats {
  data: Vec<f64>,
}

impl DiscreteStats {
  pub fn new(data: Vec<f64>) -> DiscreteStats {
    DiscreteStats { data }
  }
}

#[gql::Object]
impl DiscreteStats {
  async fn frequency(&self) -> usize {
    self.data.len()
  }

  async fn min(&self) -> f64 {
    Statistics::min(self.data.iter())
  }

  async fn max(&self) -> f64 {
    Statistics::max(self.data.iter())
  }

  async fn mean(&self) -> f64 {
    Statistics::mean(self.data.iter())
  }

  async fn std_dev(&self) -> f64 {
    Statistics::std_dev(self.data.iter())
  }

  async fn population_std_dev(&self) -> f64 {
    Statistics::population_std_dev(self.data.iter())
  }

  async fn variance(&self) -> f64 {
    Statistics::variance(self.data.iter())
  }

  async fn population_variance(&self) -> f64 {
    Statistics::population_variance(self.data.iter())
  }
}
