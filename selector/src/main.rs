
use tokio;
use tokio::sync::mpsc::channel;
use std::{ thread, time::Duration };
use std::sync::Once;


static INIT: Once = Once::new();

#[tokio::main]
async fn main() {
    let (tx, mut rx) = channel(64);
    let mut done = false;

    let operation = switch(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(12).await;
        let _ = tx.send(5).await;
        let _ = tx.send(8).await;
        let _ = tx.send(4).await;
        let _ = tx.send(9).await;
        let _ = tx.send(13).await;
        let _ = tx.send(16).await;
    }).await.unwrap();

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                if let Some(num) = res {
                    println!("{}", num)
                }
            }
            Some(num) = rx.recv() => {
                INIT.call_once( || {
                    thread::sleep(Duration::from_secs(1));
                });
                if num % 2 == 0 {
                    operation.set(switch(Some(num)));
                    done = false;
                }
            }
        }
    }
}


async fn switch(input: Option<i32>) -> Option<i32> {
    let n = match input {
        Some(num) => {
            num
        },
        None => {
            return None
        }
    };

    let calc = n * n;
    Some(calc)
}
