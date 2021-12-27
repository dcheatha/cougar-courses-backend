use lib::init;
use lib::model::db::{CoursesActiveModel, GradesActiveModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Serialize, Deserialize)]
struct CourseRow {
  pub year: i16,
  pub semester: String,
  pub campus: String,
  pub academic_group: String,
  pub subject: String,
  pub catalog_no: i32,
  pub section: String,
  pub course_no: i32,
  pub title: String,
  pub instructor: Option<String>,
  pub headcount: i32,
  pub dropped: i32,
  pub meeting_times: Option<String>,
  pub grades: Vec<(f64, i32)>,
}

async fn import_course(database: &DatabaseConnection, data: CourseRow) {
  let course = CoursesActiveModel {
    year: Set(data.year),
    semester: Set(data.semester),
    campus: Set(data.campus),
    academic_group: Set(data.academic_group),
    subject: Set(data.subject),
    catalog_no: Set(data.catalog_no),
    section: Set(data.section),
    course_no: Set(data.course_no),
    title: Set(data.title),
    instructor: Set(data.instructor),
    headcount: Set(data.headcount),
    dropped: Set(data.dropped),
    meeting_times: Set(data.meeting_times),
    ..Default::default()
  };

  let course = course.insert(database).await.unwrap();

  let mut grades = vec![];

  for (grade, headcount) in data.grades {
    let grade = GradesActiveModel {
      course_id: course.id.clone(),
      grade: Set(grade),
      headcount: Set(headcount),
      ..Default::default()
    };

    grades.push(grade.insert(database));
  }

  futures::future::join_all(grades).await;
}

#[actix_web::main]
async fn main() {
  let core_state = init::init().await.unwrap();
  let file = File::open("./import/data.json").unwrap();

  for line in io::BufReader::new(file).lines() {
    let line = line.unwrap();
    let data: CourseRow = serde_json::from_str(&line).unwrap();
    import_course(&core_state.database, data).await;
  }
}
