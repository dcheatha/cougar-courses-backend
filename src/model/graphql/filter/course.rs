use async_graphql::InputObject;
use sea_orm::sea_query::Cond;

use super::FilterOptions;
use crate::model::db::CoursesColumn;

#[derive(InputObject, Default)]
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
  pub fn to(self) -> sea_orm::Condition {
    let mut c = Cond::all();

    if let Some(filter) = self.id {
      c = c.add(filter.to(CoursesColumn::Id));
    }

    if let Some(filter) = self.year {
      c = c.add(filter.to(CoursesColumn::Year));
    }

    if let Some(filter) = self.semester {
      c = c.add(filter.to(CoursesColumn::Semester));
    }

    if let Some(filter) = self.campus {
      c = c.add(filter.to(CoursesColumn::Campus));
    }

    if let Some(filter) = self.academic_group {
      c = c.add(filter.to(CoursesColumn::AcademicGroup));
    }

    if let Some(filter) = self.subject {
      c = c.add(filter.to(CoursesColumn::Subject));
    }

    if let Some(filter) = self.catalog_no {
      c = c.add(filter.to(CoursesColumn::CatalogNo));
    }

    if let Some(filter) = self.section {
      c = c.add(filter.to(CoursesColumn::Section));
    }

    if let Some(filter) = self.course_no {
      c = c.add(filter.to(CoursesColumn::CourseNo));
    }

    if let Some(filter) = self.title {
      c = c.add(filter.to(CoursesColumn::Title));
    }

    if let Some(filter) = self.instructor {
      c = c.add(filter.to(CoursesColumn::Instructor));
    }

    if let Some(filter) = self.headcount {
      c = c.add(filter.to(CoursesColumn::Headcount));
    }

    if let Some(filter) = self.dropped {
      c = c.add(filter.to(CoursesColumn::Dropped));
    }

    if let Some(filter) = self.meeting_times {
      c = c.add(filter.to(CoursesColumn::MeetingTimes));
    }

    c
  }
}
