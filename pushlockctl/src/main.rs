use args::{Action, Config};
use pushlock_lib::{Locked, UserInfo};
use reqwest::{Client, Method, Response, StatusCode};
use std::process;

mod args;
mod colors;

fn print_ok() {
    colors::green(|| println!("OK"))
}

fn print_unknown_error(res: Response) {
    colors::red(|| {
        println!("Unknown error");
        println!("{:?}", res);
    })
}

async fn print_locked(res: Response) {
    let result: Locked = res.json().await.unwrap();

    if !result.push_available {
        colors::red(|| println!("Locked by: {}", result.locked_by.expect("username")));
        process::exit(1);
    } else if let Some(_locked_by) = result.locked_by {
        colors::yellow(|| println!("Locked by me"));
        process::exit(2);
    } else {
        colors::green(|| println!("Push is available"));
        process::exit(0);
    }
}

#[tokio::main]
async fn main() {
    let args = Config::from_config();
    let client = Client::new();

    let user_info = UserInfo {
        username: args.username,
    };

    let req = client
        .request(Method::POST, &args.endpoint)
        .json(&user_info);
    let res = req.send().await.unwrap();

    match args.action {
        Action::Lock | Action::Release => match res.status() {
            StatusCode::OK => print_ok(),
            StatusCode::LOCKED => print_locked(res).await,
            _ => print_unknown_error(res),
        },
        Action::CheckState => match res.status() {
            StatusCode::OK | StatusCode::LOCKED => {
                print_locked(res).await;
            }
            _ => {
                print_unknown_error(res);
                process::exit(255)
            }
        },
    }
}
