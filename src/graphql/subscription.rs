use futures::{
    task::{Context as StreamContext, Poll},
    Stream,
};
use juniper::{graphql_subscription, FieldError, FieldResult};
use std::pin::Pin;

use super::Context;
use crate::repository::Message;

pub struct Subscription;

// TODO: Move this into
#[derive(Clone)]
struct FutureReceiver<T> {
    receiver: crossbeam_channel::Receiver<T>,
}

impl<T> Stream for FutureReceiver<T> {
    type Item = Result<T, FieldError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut StreamContext<'_>) -> Poll<Option<Self::Item>> {
        match self.receiver.try_recv() {
            Ok(item) => Poll::Ready(Some(Ok(item))),
            Err(crossbeam_channel::TryRecvError::Empty) => Poll::Pending,
            Err(crossbeam_channel::TryRecvError::Disconnected) => Poll::Ready(None),
        }
    }
}

type MessageStream = Pin<Box<FutureReceiver<Message>>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn get_messages(ctx: &Context) -> FieldResult<MessageStream> {
        let receiver = ctx.repo.register_listener()?;
        let stream = FutureReceiver { receiver };
        Ok(Box::pin(stream))
    }
}
