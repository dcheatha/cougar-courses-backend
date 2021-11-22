use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "courses")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub year: i16,
  #[sea_orm(column_type = "Text")]
  pub semester: String,
  #[sea_orm(column_type = "Text")]
  pub campus: String,
  #[sea_orm(column_type = "Text")]
  pub academic_group: String,
  #[sea_orm(column_type = "Text")]
  pub subject: String,
  pub catalog_no: i32,
  #[sea_orm(column_type = "Text")]
  pub section: String,
  pub course_no: i32,
  #[sea_orm(column_type = "Text")]
  pub title: String,
  #[sea_orm(column_type = "Text", nullable)]
  pub instructor: Option<String>,
  pub headcount: i32,
  pub dropped: i32,
  #[sea_orm(column_type = "Text", nullable)]
  pub meeting_times: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::grades::Entity")]
  Grades,
}

impl Related<super::grades::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Grades.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
