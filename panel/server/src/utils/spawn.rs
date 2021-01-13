use tokio::task;
use std::future::Future;
use futures::future::{Abortable, AbortHandle};
use tokio::sync::oneshot::channel;

fn spawn_future_with_abort(fut: impl Future<Output=()> + Send + 'static) -> AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(fut, abort_registration);

    task::spawn(future);

    abort_handle
}

pub struct SpawnOwner {
    abort_handle: AbortHandle,
}

impl SpawnOwner {
    pub fn new(fut: impl Future<Output=()> + Send + 'static) -> SpawnOwner {
        SpawnOwner {
            abort_handle: spawn_future_with_abort(fut),
        }
    }

    pub fn off(self) {
    }
}

impl Drop for SpawnOwner {
    fn drop(&mut self) {
        self.abort_handle.abort();
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
