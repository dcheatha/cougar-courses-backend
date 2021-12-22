use async_graphql::InputObject;

use super::FilterOptions;

#[derive(InputObject)]
pub struct Course {
  id: FilterOptions<i32>,
  year: FilterOptions<i16>,
  semester: FilterOptions<String>,
  campus: FilterOptions<String>,
  academic_group: FilterOptions<String>,
  subject: FilterOptions<String>,
  catalog_no: FilterOptions<i32>,
  section: FilterOptions<String>,
  course_no: FilterOptions<i32>,
  title: FilterOptions<String>,
  instructor: FilterOptions<String>,
  headcount: FilterOptions<i32>,
  dropped: FilterOptions<i32>,
  meeting_times: FilterOptions<String>,
}

impl Course {
  fn to() -> sea_orm::Condition {
    unimplemented!()
  }
}