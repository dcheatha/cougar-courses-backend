use async_graphql as gql;
use std::sync::Arc;

use crate::model::{
  app::{self, CoreResult},
  db::*,
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
}
