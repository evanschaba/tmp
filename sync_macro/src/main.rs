use synco::sync;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::task;
use tokio::time::{sleep, Duration};

#[sync]
pub struct MyStruct {
    pub a: u32,
    #[sync]
    pub b: u32,
}

pub async fn a(state: Arc<MyStructState>) {
    let mut rx = state.b.subscribe();
    loop {
        rx.changed().await.unwrap();
        println!("a() detected update: {}", *rx.borrow());
    }
}

pub async fn B(state: Arc<MyStructState>) {
    let mut rx = state.b.subscribe();
    loop {
        rx.changed().await.unwrap();
        println!("B() detected update: {}", *rx.borrow());
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(MyStructState::new());

    let a_task = {
        let state = state.clone();
        task::spawn(a(state))
    };

    let b_task = {
        let state = state.clone();
        task::spawn(B(state))
    };

    sleep(Duration::from_secs(1)).await;
    state.set_b(42);

    sleep(Duration::from_secs(1)).await;
    state.set_b(100);

    a_task.await.unwrap();
    b_task.await.unwrap();
}
 