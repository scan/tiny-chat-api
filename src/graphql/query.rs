use juniper::graphql_object;

use super::context::Context;

pub struct Query;

#[graphql_object(
    Context = Context,
  )]
impl Query {
    fn apiVersion() -> &str {
        "0.1"
    }
}
