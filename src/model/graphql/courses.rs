use async_graphql as gql;
use std::sync::Arc;

use crate::{
  model::{
    app::{self, CoreResult},
    db::*,
  },
  stats,
};

#[gql::ComplexObject]
impl courses::Model {
  async fn grades(&self, ctx: &gql::Context<'_>) -> CoreResult<Vec<grades::Model>> {
    let state = ctx.data::<Arc<app::CoreState>>().unwrap();

    if let Some(grades) = state.dataloader.grades.load_one(self.id).await? {
      Ok(grades)
    } else {
      Ok(vec![])
    }
  }

  async fn grade_stats(&self, ctx: &gql::Context<'_>) -> CoreResult<stats::DiscreteStats> {
    let state = ctx.data::<Arc<app::CoreState>>().unwrap();

    let grades: Vec<f64> = state
      .dataloader
      .grades
      .load_one(self.id)
      .await?
      .unwrap_or_default()
      .into_iter()
      .map(|grade| vec![grade.grade; grade.headcount as usize])
      .flatten()
      .collect();

    Ok(stats::DiscreteStats::new(grades))
  }
}
