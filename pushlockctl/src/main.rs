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
            let req = client.post(&args.endpoint).send().await.unwrap();

            match req.status() {
                StatusCode::OK => println!("done"),
                StatusCode::LOCKED => println!("already locked by: {}", req.text().await.unwrap()),
                _ => println!("unknown error"),
            }
        }
        Action::CheckState => {
            let req = client.get(&args.endpoint).send().await.unwrap();

            match req.status() {
                StatusCode::OK => {
                    println!("unlocked");
                    process::exit(0);
                }
                StatusCode::LOCKED => {
                    println!("locked by: {}", req.text().await.unwrap());
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
