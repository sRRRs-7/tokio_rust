use tokio;
use tokio::io;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> io::Result<()> {
    //server
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("socket: {:?}", socket);
        println!("addr: {:?}", addr);

        let (mut rd, mut wr) = socket.split();

        if io::copy(&mut rd, &mut wr).await.is_err() {
            eprintln!("failed to copy")
        }
    }
}
