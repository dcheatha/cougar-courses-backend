use std::sync::Arc;
use async_graphql as gql;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder};

use crate::model::app::{self, CoreResult};
use crate::model::db::{CoursesModel, Grades, GradesColumn, GradesModel};

#[gql::ComplexObject]
impl CoursesModel {
  async fn grades(&self, ctx: &gql::Context<'_>) -> CoreResult<Vec<GradesModel>> {
    let state = ctx.data::<Arc<app::CoreState>>().unwrap();

    let grades = Grades::find()
      .filter(GradesColumn::CourseId.eq(self.id))
      .order_by_asc(GradesColumn::Grade)
      .all(&state.database)
      .await
      .unwrap();

    Ok(grades)
  }
}