use rand::Rng;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

/// accept 1 message req & echo back
async fn basic_echo_server() {
    let addr = "0.0.0.0:8081";
    let listener = TcpListener::bind(addr).await.unwrap();

    let (mut tcp_stream, _socket_addr) = listener.accept().await.unwrap();

    // read data received from client into here
    let mut buf = [0u8; 1024]; // 1kb buf
    let bytes_read = tcp_stream.read(&mut buf).await.unwrap();

    tcp_stream.write_all(&buf[..bytes_read]).await.unwrap();
}

#[tokio::main]
async fn main() {
    let _ = rand::thread_rng().gen_range::<u8, _>(0..=16);
    basic_echo_server().await;
}
