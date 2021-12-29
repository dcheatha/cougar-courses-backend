use crate::model::db;
use async_graphql as gql;

use super::{CategoricalStats, DiscreteStats};

pub struct AggregateCourseStats {
  data: Vec<db::courses::Model>,
}

impl AggregateCourseStats {
  pub fn new(data: Vec<db::courses::Model>) -> AggregateCourseStats {
    AggregateCourseStats { data }
  }
}

macro_rules! build_data {
  ($struct:expr, $field:ident) => {{
    $struct.iter().map(|datum| datum.$field.clone()).collect()
  }};
}

macro_rules! build_categorical_stats {
  ($data:expr, $kind:ident, $field:ident) => {{
    CategoricalStats::<$kind>::new(build_data!($data, $field))
  }};
}

macro_rules! build_discrete_stats {
  ($data:expr, $field:ident) => {{
    DiscreteStats::new($data.iter().map(|datum| datum.$field as f64).collect())
  }};
}

#[gql::Object]
impl AggregateCourseStats {
  async fn year(&self) -> CategoricalStats<i16> {
    build_categorical_stats!(self.data, i16, year)
  }

  async fn semester(&self) -> CategoricalStats<String> {
    build_categorical_stats!(self.data, String, semester)
  }

  async fn headcount(&self) -> DiscreteStats {
    build_discrete_stats!(self.data, headcount)
  }
}
