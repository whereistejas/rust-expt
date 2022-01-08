use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl TimerFuture {
    // this method is the actual computation we are performing in the Future.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            };
        });

        TimerFuture { shared_state }
    }
}

// This struct wraps the computation in a Future.
pub struct TimerFuture {
    // We are going with Arc<Mutex<T>> because Arc makes moving references of type `T`
    // across thread-safe, and Mutex make the type `T` thread-safe.
    shared_state: Arc<Mutex<SharedState>>,
}

// Here we define how the async executor will know if the computation is complete, i.e., the Future
// is ready to be polled again.
// IMPORTANT: The waker has to be updated every single time after we call the Future.
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed {
            Poll::Ready(())
        } else {
            // The waker is updated everytime the future is polled.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
