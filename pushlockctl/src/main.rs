use args::{Action, Config};
use colored::*;
use hyper::{Body, Client, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::process;

mod args;

#[derive(Deserialize)]
struct Locked {
    push_available: bool,
    locked_by: Option<String>,
}

#[derive(Serialize)]
struct UserInfo {
    username: String,
}

fn print_ok() {
    println!("{}", "OK".green());
}

fn print_unknown_error() {
    println!("{}", "Unknown error".red());
}

async fn print_locked(res: Response<Body>) {
    let buf = hyper::body::to_bytes(res).await.expect("body");
    let result: Locked = serde_json::from_slice(&buf).expect("json");

    if !result.push_available {
        println!("{} {}", "Locked by:".red(), result.locked_by.unwrap());
        process::exit(1);
    } else if let Some(_locked_by) = result.locked_by {
        println!("{}", "Locked by me".yellow());
        process::exit(2);
    } else {
        println!("{}", "Push is available".green());
        process::exit(0);
    }
}

#[tokio::main]
async fn main() {
    let args = Config::from_config();
    let client = Client::new();

    let method = match args.action {
        Action::Lock | Action::Release => Method::POST,
        Action::CheckState => Method::GET,
    };

    let user_info = UserInfo {
        username: args.username.clone(),
    };
    let bytes = serde_json::to_vec(&user_info).unwrap();

    let req = Request::builder()
        .uri(&args.endpoint)
        .method(method)
        .body(Body::from(bytes))
        .unwrap();
    let res = client.request(req).await.unwrap();

    match args.action {
        Action::Lock | Action::Release => match res.status() {
            StatusCode::OK => print_ok(),
            StatusCode::LOCKED => print_locked(res).await,
            _ => print_unknown_error(),
        },
        Action::CheckState => match res.status() {
            StatusCode::OK | StatusCode::LOCKED => {
                print_locked(res).await;
            }
            _ => {
                print_unknown_error();
                process::exit(255)
            }
        },
    }
}
