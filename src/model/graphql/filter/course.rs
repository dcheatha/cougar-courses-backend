use async_graphql::InputObject;
use sea_orm::{sea_query::Cond, Condition};

use super::FilterOptions;
use crate::model::db::CoursesColumn;

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
  fn to(self) -> sea_orm::Condition {
    let mut condition = Cond::all();

    if let Some(filter) = self.id {
      condition = condition.add(filter.to(CoursesColumn::Id));
    }

    if let Some(filter) = self.year {
      condition = condition.add(filter.to(CoursesColumn::Year));
    }

    condition
  }
}
