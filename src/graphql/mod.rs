mod context;
mod mutation;
mod query;
mod subscription;

pub use context::Context;

use juniper::RootNode;

type Schema = RootNode<'static, query::Query, mutation::Mutation, subscription::Subscription>;

pub fn schema() -> Schema {
    Schema::new(query::Query, mutation::Mutation, subscription::Subscription)
}
