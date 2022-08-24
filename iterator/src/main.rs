use tokio;
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut arr = stream::iter(vec![1, 2, 3, 5, 7, 9, 11]);

    while let Some(i) = arr.next().await {
        println!("{}", i);
    }
}
