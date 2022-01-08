use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    FutureExt,
};
use std::{
    future::Future,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
    time::Duration,
};
use timer_future::TimerFuture;

// any async mechanism needs a spawner, an executor and a waker.

// the spawner takes a computation and wraps it up in a future.
// the executor drives future as far as they can be pushed to completion.
// if the executor is not able to complete a future, it will have to be
// informed to repoll it, when the future is ready to be polled, this task
// is performed by the waker.

// This is the struct that will store our computation.
struct Task {
    // A `BoxFuture` is a Box with 'static + impl Future + Send
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

// This is use to create new future and add them to the task_queue.
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

// This consumes tasks from the queue and polls them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    // this method accepts a computation in the form of a future and creates a new task and adds the task to the
    // task queue.
    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_sender.send(task).expect("Too many tasks");
    }
}

// whenever an executor calls `wake()` on a task, create a copy of itself and requeue it.
// this trait is used by `waker_ref`.
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self
            .task_sender
            .send(arc_self.clone())
            .expect("Too many tasks");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                // the waker_ref method uses the `ArcWake` trait to convert the task into a waker.
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);

                // you can only call `poll` on mutable refs.
                if future.as_mut().poll(context).is_pending() {
                    // if a Future is still pending return it to its "slot"
                    // derefing a mutex gives you the data it is protecting.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("Hello");

        TimerFuture::new(Duration::new(2, 0)).await;

        println!("Done");
    });

    drop(spawner);

    executor.run();
}
