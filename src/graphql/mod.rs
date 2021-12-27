use std::sync::Arc;

use async_graphql as gql;
use gql::{Schema, EmptyMutation, EmptySubscription};
use sea_orm::{EntityTrait, QueryFilter};


use crate::model::{app, graphql::filter, db};

pub type GraphQLSchema =  Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[gql::Object]
impl Query {
  async fn get_courses(
    &self,
    ctx: &gql::Context<'_>,
    course_filter: filter::course::Course,
  ) -> app::CoreResult<Vec<db::CoursesModel>> {
    let core_state = ctx.data::<Arc<app::CoreState>>().unwrap();

    let courses = db::Courses::find()
    .filter(course_filter.to())
    .all(&core_state.database)
    .await?;

    Ok(courses)
  }
}
