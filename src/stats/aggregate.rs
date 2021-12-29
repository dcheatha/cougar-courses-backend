use std::sync::Arc;

use crate::model::{app, db};
use async_graphql as gql;

use super::{CategoricalStats, DiscreteStats};

pub struct AggregateCourseStats {
  state: Arc<app::CoreState>,
  data: Vec<db::courses::Model>,
}

impl AggregateCourseStats {
  pub fn new(state: Arc<app::CoreState>, data: Vec<db::courses::Model>) -> AggregateCourseStats {
    AggregateCourseStats { data, state }
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

macro_rules! build_optional_categorical_stats {
  ($data:expr, $kind:ident, $field:ident) => {{
    CategoricalStats::<$kind>::new(
      $data
        .iter()
        .map(|datum| datum.$field.clone())
        .flatten()
        .collect(),
    )
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

  async fn campus(&self) -> CategoricalStats<String> {
    build_categorical_stats!(self.data, String, campus)
  }

  async fn academic_group(&self) -> CategoricalStats<String> {
    build_categorical_stats!(self.data, String, academic_group)
  }

  async fn subject(&self) -> CategoricalStats<String> {
    build_categorical_stats!(self.data, String, subject)
  }

  async fn catalog_no(&self) -> CategoricalStats<i32> {
    build_categorical_stats!(self.data, i32, catalog_no)
  }

  async fn section(&self) -> CategoricalStats<String> {
    build_categorical_stats!(self.data, String, section)
  }

  async fn course_no(&self) -> CategoricalStats<i32> {
    build_categorical_stats!(self.data, i32, course_no)
  }

  async fn title(&self) -> CategoricalStats<String> {
    build_categorical_stats!(self.data, String, title)
  }

  async fn instructor(&self) -> CategoricalStats<String> {
    build_optional_categorical_stats!(self.data, String, instructor)
  }

  async fn headcount(&self) -> DiscreteStats {
    build_discrete_stats!(self.data, headcount)
  }

  async fn dropped(&self) -> DiscreteStats {
    build_discrete_stats!(self.data, dropped)
  }

  async fn meeting_times(&self) -> CategoricalStats<String> {
    build_optional_categorical_stats!(self.data, String, meeting_times)
  }

  async fn grades(&self) -> DiscreteStats {
    let ids = self.data.iter().map(|course| course.id);

    let grades = self
      .state
      .dataloader
      .grades
      .load_many(ids)
      .await
      .unwrap_or_default()
      .values()
      .flatten()
      .map(|grade| vec![grade.grade; grade.headcount as usize])
      .flatten()
      .collect();

    DiscreteStats::new(grades)
  }
}
