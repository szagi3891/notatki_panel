use tokio::task;
use std::future::Future;
use futures::future::{Abortable, AbortHandle};

fn spawnFutureWithAbort(fut: impl Future<Output=()> + Send + 'static) -> AbortHandle {
    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    let future = Abortable::new(fut, abort_registration);

    task::spawn(future);

    abort_handle
}

struct Task {
    abortHandle: AbortHandle,
}

impl Task {
    pub fn new(fut: impl Future<Output=()> + Send + 'static) -> Task {
        Task {
            abortHandle: spawnFutureWithAbort(fut),
        }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        self.abortHandle.abort();
    }
}

pub struct SpawnOwner {
    child: Vec<Task>,
}

impl SpawnOwner {
    pub fn new(fut: impl Future<Output=()> + Send + 'static) -> SpawnOwner {
        let task = Task::new(fut);

        SpawnOwner {
            child: vec!(task),
        }
    }

    pub fn add_child(&mut self, mut spawn: SpawnOwner) {
        self.child.append(&mut spawn.child);
    }

    pub fn off(self) {
    }
}

