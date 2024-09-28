use rand::Rng;
#[tokio::main]
async fn main() {
    let _ = rand::thread_rng().gen_range::<u8, _>(0..=16);
    basic_echo_server().await;
}
