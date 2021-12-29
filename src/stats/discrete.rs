use async_graphql as gql;

pub struct DiscreteStats {
  data: Vec<f64>,
}

#[gql::Object]
impl DiscreteStats {
  async fn frequency(&self) -> usize {
    self.data.len()
  }
}
