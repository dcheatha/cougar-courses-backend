use async_graphql as gql;
use itertools::Itertools;

pub trait CategoricalTrait:
  Send + Sync + std::hash::Hash + std::cmp::Eq + Clone + async_graphql::OutputType
{
}

impl CategoricalTrait for String {}
impl CategoricalTrait for i16 {}
impl CategoricalTrait for i32 {}

pub struct CategoricalStats<T: CategoricalTrait> {
  data: Vec<T>,
}

#[gql::Object]
impl<T: CategoricalTrait> CategoricalStats<T> {
  async fn unique(&self) -> usize {
    self.data.iter().unique().count()
  }
}
