use async_graphql as gql;
use itertools::Itertools;

pub trait CategoricalTrait:
  Send + Sync + std::hash::Hash + std::cmp::Eq + Clone + async_graphql::OutputType
{
}

impl CategoricalTrait for String {}
impl CategoricalTrait for i16 {}
impl CategoricalTrait for i32 {}

#[derive(gql::SimpleObject)]
#[graphql(concrete(name = "CategoricalStatsI16", params(i16)))]
#[graphql(concrete(name = "CategoricalStatsI32", params(i32)))]
#[graphql(concrete(name = "CategoricalStatsString", params(String)))]
#[graphql(complex)]
pub struct CategoricalStats<T: CategoricalTrait> {
  data: Vec<T>,
}

impl<T: CategoricalTrait> CategoricalStats<T> {
  pub fn new(data: Vec<T>) -> CategoricalStats<T> {
    CategoricalStats {
      data
    }
  }
}

#[gql::ComplexObject]
impl<T: CategoricalTrait> CategoricalStats<T> {
  async fn unique(&self) -> usize {
    self.data.iter().unique().count()
  }
}
