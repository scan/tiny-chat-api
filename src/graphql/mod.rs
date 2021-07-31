mod context;
mod mutation;
mod query;

pub use context::Context;

use juniper::{EmptySubscription, RootNode};

type Schema = RootNode<'static, query::Query, mutation::Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(query::Query, mutation::Mutation, EmptySubscription::new())
}
