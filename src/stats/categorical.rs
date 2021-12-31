use std::collections::HashMap;

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
    CategoricalStats { data }
  }
}

#[derive(gql::SimpleObject)]
#[graphql(concrete(name = "BinnedItemI16", params(i16)))]
#[graphql(concrete(name = "BinnedItemI32", params(i32)))]
#[graphql(concrete(name = "BinnedItemString", params(String)))]
pub struct BinnedItem<T: CategoricalTrait>
where
  T: gql::OutputType,
{
  pub value: T,
  pub frequency: usize,
}

#[gql::ComplexObject]
impl<T: CategoricalTrait> CategoricalStats<T>
where
  BinnedItem<T>: gql::OutputType,
{
  async fn unique(&self) -> usize {
    self.data.iter().unique().count()
  }

  async fn first(&self) -> Option<T> {
    self.data.clone().into_iter().next()
  }

  async fn bin(&self) -> Vec<BinnedItem<T>> {
    let mut map: HashMap<T, usize> = HashMap::new();

    for datum in &self.data {
      let total = 1 + *map.get(datum).unwrap_or(&0);
      map.insert((*datum).clone(), total);
    }

    let mut data: Vec<BinnedItem<T>> = map
      .into_iter()
      .map(|(value, frequency)| BinnedItem { value, frequency })
      .collect();

    data.sort_by(|a, b| b.frequency.cmp(&a.frequency));

    data
  }
}
