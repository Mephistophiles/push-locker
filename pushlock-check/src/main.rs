// use reqwest::StatusCode;
use hyper::{Client, StatusCode, Uri};
use std::{env, process, str::FromStr};

#[tokio::main]
async fn main() {
    let server = env::args().nth(1).expect("Server address");
    let endpoint = format!("http://{}/is_locked", server);
    let client = Client::new();
    let res = client.get(Uri::from_str(&endpoint).unwrap()).await.unwrap();

    match res.status() {
        StatusCode::LOCKED => process::exit(1),
        _ => process::exit(0),
    }
}
