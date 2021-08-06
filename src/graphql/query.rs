use juniper::{graphql_object, FieldResult, FieldError};

use crate::repository::Message;

use super::context::Context;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct Query;

#[graphql_object(
    Context = Context,
  )]
impl Query {
    fn api_version() -> &str {
        VERSION.unwrap_or("unknown")
    }

    fn messages(context: &Context) -> FieldResult<Vec<Message>> {
        if context.user_name.is_none() {
            return Err(FieldError::from("login required"));
        }

        let messages = context.repo.get_messages(None)?;

        Ok(messages)
    }
}
