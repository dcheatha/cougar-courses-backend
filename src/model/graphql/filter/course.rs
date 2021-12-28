use async_graphql::InputObject;
use sea_orm::sea_query::Cond;

use super::FilterOptions;
use crate::model::db::courses::Column;

#[derive(InputObject, Default)]
pub struct CourseFilter {
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

impl CourseFilter {
  pub fn to(self) -> sea_orm::Condition {
    let mut c = Cond::all();

    if let Some(filter) = self.id {
      c = c.add(filter.to(Column::Id));
    }

    if let Some(filter) = self.year {
      c = c.add(filter.to(Column::Year));
    }

    if let Some(filter) = self.semester {
      c = c.add(filter.to(Column::Semester));
    }

    if let Some(filter) = self.campus {
      c = c.add(filter.to(Column::Campus));
    }

    if let Some(filter) = self.academic_group {
      c = c.add(filter.to(Column::AcademicGroup));
    }

    if let Some(filter) = self.subject {
      c = c.add(filter.to(Column::Subject));
    }

    if let Some(filter) = self.catalog_no {
      c = c.add(filter.to(Column::CatalogNo));
    }

    if let Some(filter) = self.section {
      c = c.add(filter.to(Column::Section));
    }

    if let Some(filter) = self.course_no {
      c = c.add(filter.to(Column::CourseNo));
    }

    if let Some(filter) = self.title {
      c = c.add(filter.to(Column::Title));
    }

    if let Some(filter) = self.instructor {
      c = c.add(filter.to(Column::Instructor));
    }

    if let Some(filter) = self.headcount {
      c = c.add(filter.to(Column::Headcount));
    }

    if let Some(filter) = self.dropped {
      c = c.add(filter.to(Column::Dropped));
    }

    if let Some(filter) = self.meeting_times {
      c = c.add(filter.to(Column::MeetingTimes));
    }

    c
  }
}
