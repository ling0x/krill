// Typed mailbox trait
use std::fmt::Debug;

/// Marker trait for messages that can be sent to actors
pub trait Message: Send + Sync + 'static + Debug + Clone {}

/// Mailbox abstraction (currently just the trait; implementation is in actor.rs via channels)
pub trait Mailbox<T: Message> {
    fn send(&self, msg: T) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
