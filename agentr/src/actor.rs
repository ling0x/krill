// Actor system implementation
use crate::mailbox::Message;
use anyhow::Result;
use tokio::sync::mpsc;

/// Typed reference to an actor
#[derive(Clone)]
pub struct ActorRef<T: Message> {
    tx: mpsc::Sender<T>,
}

impl<T: Message> ActorRef<T> {
    pub fn new(tx: mpsc::Sender<T>) -> Self {
        Self { tx }
    }

    /// Send a message to the actor (async)
    pub async fn send(&self, msg: T) -> Result<()> {
        self.tx.send(msg).await?;
        Ok(())
    }

    /// Try to send without blocking
    pub fn try_send(&self, msg: T) -> Result<()> {
        self.tx.try_send(msg)?;
        Ok(())
    }
}

/// Handle to a running actor
pub struct ActorHandle {
    handle: tokio::task::JoinHandle<()>,
}

impl ActorHandle {
    pub async fn join(self) -> Result<()> {
        self.handle.await?;
        Ok(())
    }
}

/// Spawn an actor with a message handler
pub fn spawn_actor<T, F, Fut>(mailbox_size: usize, handler: F) -> (ActorRef<T>, ActorHandle)
where
    T: Message,
    F: Fn(T) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<()>> + Send,
{
    let (tx, mut rx) = mpsc::channel::<T>(mailbox_size);
    let actor_ref = ActorRef::new(tx);

    let handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = handler(msg).await {
                eprintln!("Actor handler error: {}", e);
            }
        }
    });

    (actor_ref, ActorHandle { handle })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestMsg(i32);

    impl Message for TestMsg {}

    #[tokio::test]
    async fn test_actor_send_receive() {
        let (actor_ref, _handle) = spawn_actor(10, |msg: TestMsg| async move {
            assert_eq!(msg.0, 42);
            Ok(())
        });

        actor_ref.send(TestMsg(42)).await.unwrap();
    }
}
