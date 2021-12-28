use async_graphql as gql;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, gql::SimpleObject)]
#[graphql(concrete(name = "Grade", params()))]
#[sea_orm(table_name = "grades")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub course_id: i32,
  pub grade: f64,
  pub headcount: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::courses::Entity",
    from = "Column::CourseId",
    to = "super::courses::Column::Id",
    on_update = "NoAction",
    on_delete = "NoAction"
  )]
  Courses,
}

impl Related<super::courses::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Courses.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

// impl Linked for Model {
//   type FromEntity = super::Courses;
//   type ToEntity = Entity;

//   fn link(&self) -> Vec<sea_orm::LinkDef> {
//     vec![
//       Relation::Courses.def().rev(),
//       Relation::Grades.def(),
//     ]
//   }
// }
