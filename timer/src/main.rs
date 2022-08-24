use std::{
    future::Future,
    pin::Pin,
    sync::{ Arc, Mutex },
    task::{ Context, Poll, Waker },
    thread,
    time::Duration,
};
use tokio;

struct SharedState {
    id: i32,
    completed: bool,
    waker: Option<Waker>
}
struct FutureTimer {
    state: Arc<Mutex<SharedState>>
}

impl FutureTimer {
    fn new(id: i32, duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(SharedState {
            id,
            completed: false,
            waker: None,
        }));

        let thread_state = state.clone();
        thread::spawn( move || {
            thread::sleep(duration);

            let mut state = thread_state.lock().unwrap();
            state.completed = true;
            if let Some(waker) = state.waker.take() {
                waker.wake()
            }
            println!("done: {}", state.id);
        });

        FutureTimer { state }
    }
}

impl Future for FutureTimer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if state.completed {
            Poll::Ready(())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}


#[tokio::main]
async fn main() {
    let timer1 = FutureTimer::new(1, Duration::from_secs(1));
    let timer2 = FutureTimer::new(2, Duration::from_secs(2));
    let timer3 = FutureTimer::new(3, Duration::from_secs(3));

    timer3.await;
    timer2.await;
    timer1.await;
}

