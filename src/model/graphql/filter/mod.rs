use async_graphql as gql;
use sea_orm::{sea_query::Cond, ColumnTrait, Condition};

pub mod course;

pub type FilterOptions<T> = Option<Filter<T>>;

#[derive(gql::InputObject, Default)]
#[graphql(concrete(name = "FilterI16", params(i16)))]
#[graphql(concrete(name = "FilterI32", params(i32)))]
#[graphql(concrete(name = "FilterI64", params(f64)))]
#[graphql(concrete(name = "FilterString", params(String)))]
pub struct Filter<T: gql::InputType> {
  eq: Option<Vec<T>>,
  gt: Option<T>,
  gte: Option<T>,
  lt: Option<T>,
  lte: Option<T>,
}

impl<T: gql::InputType> Filter<T>
where
  sea_orm::Value: From<T>,
{
  pub fn to<C: ColumnTrait>(self, column: C) -> Condition {
    let mut condition = Cond::all();

    if let Some(value) = self.eq {
      condition = condition.add(column.is_in(value));
    }

    if let Some(value) = self.gt {
      condition = condition.add(column.gt(value));
    }

    if let Some(value) = self.lt {
      condition = condition.add(column.lt(value));
    }

    if let Some(value) = self.gte {
      condition = condition.add(column.gte(value));
    }

    if let Some(value) = self.lte {
      condition = condition.add(column.lte(value));
    }

    condition
  }
}
