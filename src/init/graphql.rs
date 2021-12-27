use async_graphql as gql;
use gql::{EmptyMutation, EmptySubscription, Schema};

use crate::graphql::{GraphQLSchema, Query};

pub fn init() -> GraphQLSchema {
  Schema::build(Query, EmptyMutation, EmptySubscription)
    .extension(gql::extensions::Analyzer)
    .extension(gql::extensions::Logger)
    .extension(gql::extensions::ApolloTracing)
    .finish()
}
