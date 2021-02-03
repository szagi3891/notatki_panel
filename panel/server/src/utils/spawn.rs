use tokio::task::{self, JoinHandle};
use std::future::Future;
use tokio::sync::oneshot::channel;

pub struct SpawnOwner {
    handler: JoinHandle<()>,
}

impl SpawnOwner {
    pub fn new(future: impl Future<Output=()> + Send + 'static) -> SpawnOwner {
        SpawnOwner {
            handler: task::spawn(future)
        }
    }

    pub fn off(self) {
    }
}

impl Drop for SpawnOwner {
    fn drop(&mut self) {
        self.handler.abort();
    }
}

pub async fn spawn_and_wait(fut: impl Future<Output=()> + Send + 'static) {

    let (sender, receiver) = channel::<()>();

    let spawn_owner = SpawnOwner::new(async move {
        fut.await;
        let _ = sender.send(());
    });

    let _ = receiver.await;
    spawn_owner.off();
}
