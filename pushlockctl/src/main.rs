use args::{Action, Config};
use pushlock_lib::{Locked, UserInfo};
use reqwest::{Client, Method, Response, StatusCode};
use std::io::Write;
use std::process;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod args;

fn red() -> StandardStream {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();

    stdout
}

fn green() -> StandardStream {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .unwrap();

    stdout
}

fn yellow() -> StandardStream {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
        .unwrap();

    stdout
}

fn print_ok() {
    let mut stdout = green();

    writeln!(&mut stdout, "OK").unwrap();
}

fn print_unknown_error(res: Response) {
    let mut stdout = red();

    writeln!(&mut stdout, "Unknown error").unwrap();
    writeln!(&mut stdout, "{:?}", res).unwrap();
}

async fn print_locked(res: Response) {
    let result: Locked = res.json().await.unwrap();

    if !result.push_available {
        let mut stdout = red();
        writeln!(
            &mut stdout,
            "Locked by: {}",
            result.locked_by.expect("username")
        )
        .unwrap();
        process::exit(1);
    } else if let Some(_locked_by) = result.locked_by {
        let mut stdout = yellow();
        writeln!(&mut stdout, "Locked by me").unwrap();
        process::exit(2);
    } else {
        let mut stdout = green();
        writeln!(&mut stdout, "Push is available").unwrap();
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
        username: whoami::username(),
    };

    let req = client.request(method, &args.endpoint).json(&user_info);
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
