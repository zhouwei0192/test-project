




#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::https("http://45.139.132.19:58000").unwrap())
        .build()
        .unwrap();

    let resp = client
        .get("https://ifconfig.me")
        .send()
        .await
        .unwrap();

    let ip = resp.text().await.unwrap();
    println!("Outgoing IP: {}", ip.trim());
}