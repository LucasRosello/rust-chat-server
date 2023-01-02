#[tokio::main]
async fn main() {

    let listener: TcpListener = TcpListener::bind("localhost:8080").await;

    let (mut socket: TcpListener, addr: SocketAddr) = listener.accept().await.unwrap();

    let mut buffer: [u8; _] = [0u8; 1024];

    let bytes_read: usize = socket.read(&mut buffer).await.unwrap();

    socket.write_all()
}