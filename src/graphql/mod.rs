mod context;
mod query;

pub use context::Context;

use juniper::{EmptySubscription, EmptyMutation, RootNode};

type Schema = RootNode<
  'static,
  query::Query,
  EmptyMutation<Context>,
  EmptySubscription<Context>,
>;

pub fn schema() -> Schema {
    Schema::new(
      query::Query,
      EmptyMutation::new(),
      EmptySubscription::new(),
    )
  }