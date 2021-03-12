use std::{io, process};

use clap::{App, Arg, ArgMatches};
use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};

pub(crate) enum Action {
    Lock,
    Release,
    CheckState,
}

pub(crate) struct Config {
    pub(crate) action: Action,
    pub(crate) endpoint: String,
}

fn autocomplete(shell: &str) {
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

impl Config {
    pub(crate) fn from_args() -> Self {
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

        let endpoint = match action {
            Action::Lock => format!("http://{}/lock", ip),
            Action::Release => format!("http://{}/release", ip),
            Action::CheckState => format!("http://{}/is_locked", ip),
        };

        Config { action, endpoint }
    }
}
