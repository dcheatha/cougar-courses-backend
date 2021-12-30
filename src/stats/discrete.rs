use std::collections::HashMap;

use async_graphql as gql;
use statrs::statistics::*;

pub struct DiscreteStats {
  data: Data<Vec<f64>>,
}

impl DiscreteStats {
  pub fn new(data: Vec<f64>) -> DiscreteStats {
    DiscreteStats {
      data: Data::new(data),
    }
  }
}

#[gql::Object]
impl DiscreteStats {
  async fn frequency(&self) -> usize {
    self.data.len()
  }

  async fn sum(&self) -> f64 {
    self.data.iter().sum()
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

  async fn count_eq(&self, value: f64) -> usize {
    self
      .data
      .iter()
      .filter(|datum| (**datum - value).abs() < 0.01)
      .count()
  }

  async fn count_neq(&self, value: f64) -> usize {
    self
      .data
      .iter()
      .filter(|datum| (**datum - value).abs() > 0.01)
      .count()
  }

  async fn count_gt(&self, value: f64) -> usize {
    self.data.iter().filter(|datum| **datum > value).count()
  }

  async fn count_gte(&self, value: f64) -> usize {
    self.data.iter().filter(|datum| **datum >= value).count()
  }

  async fn count_lt(&self, value: f64) -> usize {
    self.data.iter().filter(|datum| **datum < value).count()
  }

  async fn count_lte(&self, value: f64) -> usize {
    self.data.iter().filter(|datum| **datum <= value).count()
  }

  async fn bin(&self) -> Vec<BinnedFloat> {
    let mut bins: HashMap<String, BinnedFloat> = HashMap::new();
    let total_frequency = self.data.len();

    for value in self.data.iter() {
      if let Some(bin) = bins.get_mut(&*value.to_string()) {
        bin.frequency += 1;
        bin.percent = bin.frequency as f64 / total_frequency as f64;
        continue;
      } else {
        bins.insert(
          value.to_string(),
          BinnedFloat {
            value: *value,
            frequency: 1,
            percent: 1.0 / total_frequency as f64,
          },
        );
      }
    }

    bins.into_iter().map(|(_, value)| value).collect()
  }
}

#[derive(gql::SimpleObject)]
pub struct BinnedFloat {
  pub value: f64,
  pub frequency: usize,
  pub percent: f64,
}
