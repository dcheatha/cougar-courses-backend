use async_graphql as gql;

use super::{app, db};

pub mod filter;

pub struct Query;

#[gql::Object]
impl Query {
  async fn get_courses(
    &self,
    ctx: &gql::Context<'_>,
    course_filter: filter::course::Course,
  ) -> app::CoreResult<Vec<db::CoursesModel>> {
    Ok(vec![])
  }
}