use args::{Action, Config};
use reqwest::StatusCode;
use std::process;

mod args;

#[tokio::main]
async fn main() {
    let args = Config::from_args();

    let client = reqwest::Client::new();

    match args.action {
        Action::Lock | Action::Release => {
            let res = client.post(&args.endpoint).send().await.unwrap();

            match res.status() {
                StatusCode::OK => println!("done"),
                StatusCode::LOCKED => println!("already locked by: {}", res.text().await.unwrap()),
                _ => println!("unknown error"),
            }
        }
        Action::CheckState => {
            let res = client.get(&args.endpoint).send().await.unwrap();

            match res.status() {
                StatusCode::OK => {
                    println!("unlocked");
                    process::exit(0);
                }
                StatusCode::LOCKED => {
                    println!("locked by: {}", res.text().await.unwrap());
                    process::exit(1);
                }
                _ => {
                    println!("unknown error");
                    process::exit(0)
                }
            }
        }
    }
}
