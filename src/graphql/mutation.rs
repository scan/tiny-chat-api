use juniper::{graphql_object, FieldResult};

use super::context::Context;

#[derive(Debug, Copy, Clone)]
pub struct Mutation;

#[graphql_object(
    Context = Context,
  )]
impl Mutation {
    fn login(context: &Context, username: String, _server_password: String) -> FieldResult<String> {
        let token = context.auth_manager.token_for_user(&username)?;

        Ok(token)
    }
}
