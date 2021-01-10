use tokio::task;
use std::future::Future;
use futures::future::{Abortable, AbortHandle};

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