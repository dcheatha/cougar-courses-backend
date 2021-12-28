use std::collections::HashMap;

use async_graphql as gql;
use gql::async_trait;
use gql::dataloader::*;
use sea_orm::sea_query::Cond;

use crate::model::app;
use crate::model::db::*;

struct GradesDataLoader {
  core_state: app::CoreState,
}

#[async_trait::async_trait]
impl Loader<i32> for GradesDataLoader {
  type Value = grades::Model;
  type Error = app::CoreError;

  async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
    let filter = Cond::all().add(grades::Column::Id.is_in(keys.to_owned()));

    let grades = grades::Entity::find()
      .filter(filter)
      .all(&self.core_state.database)
      .await?;

    let grades: HashMap<_, _> = grades.into_iter().map(|grade| (grade.id, grade)).collect();

    Ok(grades)
  }
}
