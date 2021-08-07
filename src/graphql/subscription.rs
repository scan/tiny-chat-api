use futures::Stream;
use juniper::{FieldError, graphql_subscription};
use std::pin::Pin;

use super::Context;
use crate::repository::Message;

pub struct Subscription;

type MessageStream = Pin<Box<dyn Stream<Item = Result<Message, FieldError>> + Send>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn get_messages(ctx: &Context) -> MessageStream {
        let receiver = ctx.repo.register_listener()?;
        let stream = receiver;
        Box::pin(stream)
    }
}