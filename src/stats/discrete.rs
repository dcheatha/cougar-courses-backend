use async_graphql as gql;

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
}
