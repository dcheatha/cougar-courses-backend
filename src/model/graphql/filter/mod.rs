use async_graphql as gql;

pub mod course;

#[derive(gql::InputObject)]
#[graphql(concrete(name = "FilterI16", params(i16)))]
#[graphql(concrete(name = "FilterI32", params(i32)))]
#[graphql(concrete(name = "FilterI64", params(f64)))]
#[graphql(concrete(name = "FilterString", params(String)))]
pub struct FilterOptions<T: gql::InputType> {
  eq: Option<Vec<T>>,
  gt: Option<T>,
  lt: Option<T>,
}
