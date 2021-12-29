use std::sync::Arc;

use async_graphql as gql;
use gql::{EmptyMutation, EmptySubscription, Schema};

use crate::{
  model::{app, db::*, graphql::filter},
  stats::aggregate::AggregateCourseStats,
};

pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[gql::Object]
impl Query {
  async fn get_courses(
    &self,
    ctx: &gql::Context<'_>,
    course_filter: filter::course::CourseFilter,
  ) -> app::CoreResult<Vec<courses::Model>> {
    let core_state = ctx.data::<Arc<app::CoreState>>().unwrap();

    let courses = courses::Entity::find()
      .filter(course_filter.to())
      .all(&*core_state.database)
      .await?;

    Ok(courses)
  }

  async fn get_aggregate_stats(
    &self,
    ctx: &gql::Context<'_>,
    course_filter: filter::course::CourseFilter,
  ) -> app::CoreResult<AggregateCourseStats> {
    let core_state = ctx.data::<Arc<app::CoreState>>().unwrap();

    let courses = courses::Entity::find()
      .filter(course_filter.to())
      .all(&*core_state.database)
      .await?;

    Ok(AggregateCourseStats::new(courses))
  }
}
