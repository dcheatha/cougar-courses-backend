pub mod courses;
pub mod grades;

pub use sea_orm::{
  ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
