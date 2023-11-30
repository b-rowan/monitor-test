use monitor_test::get_miner;

#[tokio::main]
async fn main() {
    let miner_ip = String::from("127.0.0.1");

    let miner_type = get_miner(&miner_ip).await.unwrap();

    dbg!(miner_type);
}