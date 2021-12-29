use async_graphql as gql;
use statrs::statistics::*;

pub struct DiscreteStats {
  data: Data<Vec<f64>>,
}

impl DiscreteStats {
  pub fn new(data: Vec<f64>) -> DiscreteStats {
    DiscreteStats { data: Data::new(data) }
  }
}

#[gql::Object]
impl DiscreteStats {
  async fn frequency(&self) -> usize {
    self.data.len()
  }

  async fn min(&self) -> f64 {
    self.data.min()
  }

  async fn max(&self) -> f64 {
    self.data.max()
  }

  async fn mean(&self) -> Option<f64> {
    self.data.mean()
  }

  async fn median(&self) -> f64 {
    self.data.median()
  }

  async fn std_dev(&self) -> Option<f64> {
    self.data.std_dev()
  }

  async fn variance(&self) -> Option<f64> {
    self.data.variance()
  }

  async fn lower_quartile(&self) -> f64 {
    self.data.clone().lower_quartile()
  }

  async fn upper_quartile(&self) -> f64 {
    self.data.clone().upper_quartile()
  }
}
