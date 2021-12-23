use async_graphql as gql;
use gql::{Schema, EmptyMutation, EmptySubscription};

use crate::model::graphql::{Query, GraphQLSchema};

pub fn init() -> GraphQLSchema {
  Schema::build(Query, EmptyMutation, EmptySubscription)
  .extension(gql::extensions::Analyzer)
  .extension(gql::extensions::Logger)
  .extension(gql::extensions::ApolloTracing)
  .finish()
}
