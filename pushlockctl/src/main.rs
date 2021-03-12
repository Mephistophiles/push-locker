use clap::{App, Arg, ArgMatches};
use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use std::process;

fn autocomplete(shell: &str) {
    use std::io;

    let mut app = app();

    match shell {
        "bash" => generate::<Bash, _>(&mut app, clap::crate_name!(), &mut io::stdout()),
        "elvish" => generate::<Elvish, _>(&mut app, clap::crate_name!(), &mut io::stdout()),
        "fish" => generate::<Fish, _>(&mut app, clap::crate_name!(), &mut io::stdout()),
        "powershell" => generate::<PowerShell, _>(&mut app, clap::crate_name!(), &mut io::stdout()),
        "zsh" => generate::<Zsh, _>(&mut app, clap::crate_name!(), &mut io::stdout()),
        _ => panic!("Unknown generator"),
    }
}

fn app() -> App<'static> {
    App::new("pushlock")
        .subcommand(App::new("generate").arg(Arg::new("shell").takes_value(true).required(true)))
        .subcommand(App::new("lock"))
        .subcommand(App::new("unlock"))
        .subcommand(App::new("check"))
        .arg(Arg::new("server").takes_value(true).required(true))
}

fn args() -> ArgMatches {
    app().get_matches()
}

enum Action {
    Lock,
    Release,
    CheckState,
}

#[tokio::main]
async fn main() {
    let args = args();

    if let Some(generate) = args.subcommand_matches("generate") {
        autocomplete(generate.value_of("shell").unwrap());
        process::exit(0);
    }

    let action = if args.is_present("lock") {
        Action::Lock
    } else if args.is_present("unlock") {
        Action::Release
    } else if args.is_present("check") {
        Action::CheckState
    } else {
        panic!("Invalid action");
    };

    let ip = args.value_of("server").expect("server");
    let client = reqwest::Client::new();

    let endpoint = match action {
        Action::Lock => format!("http://{}/lock", ip),
        Action::Release => format!("http://{}/release", ip),
        Action::CheckState => format!("http://{}/is_locked", ip),
    };

    match action {
        Action::Lock | Action::Release => {
            let req = client.post(&endpoint).send().await.unwrap();

            if req.status().is_success() {
                println!("done");
            } else {
                println!("already locked");
            }
        }
        Action::CheckState => {
            let req = client.get(&endpoint).send().await.unwrap();

            if req.status().is_success() {
                println!("unlocked");
                process::exit(0);
            } else {
                println!("locked");
                process::exit(1);
            }
        }
    }
}
