use args::{Action, Config};
use colored::*;
use hyper::{Body, Client, Method, Request, Response, StatusCode};
use serde::Deserialize;
use std::{net::IpAddr, process};

mod args;

#[derive(Deserialize)]
struct Locked {
    locked_by: Vec<IpAddr>,
}

fn print_ok() {
    println!("{}", "OK".green());
}

fn print_unknown_error() {
    println!("{}", "Unknown error".red());
}

fn print_available() {
    println!("{}", "Push is available".green());
}

async fn print_locked(res: Response<Body>) {
    let buf = hyper::body::to_bytes(res).await.expect("body");
    let result: Locked = serde_json::from_slice(&buf).expect("json");

    println!("{}", "Locked".yellow());

    for ip in result.locked_by {
        println!("* {}", ip);
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

    let req = Request::builder()
        .uri(&args.endpoint)
        .method(method)
        .body(Body::empty())
        .unwrap();
    let res = client.request(req).await.unwrap();

    match args.action {
        Action::Lock | Action::Release => match res.status() {
            StatusCode::OK => print_ok(),
            StatusCode::LOCKED => print_locked(res).await,
            _ => print_unknown_error(),
        },
        Action::CheckState => match res.status() {
            StatusCode::OK => {
                print_available();
                process::exit(0);
            }
            StatusCode::LOCKED => {
                print_locked(res).await;
                process::exit(1);
            }
            _ => {
                print_unknown_error();
                process::exit(2)
            }
        },
    }
}
