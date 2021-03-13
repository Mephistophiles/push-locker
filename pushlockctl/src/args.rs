use clap::{App, AppSettings, Arg, ArgMatches};
use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use indoc::indoc;
use serde::Deserialize;
use std::{
    fs, io,
    process::{self, abort},
};
use user_error::{UserFacingError, UFE};

pub(crate) enum Action {
    Lock,
    Release,
    CheckState,
}

impl Default for Action {
    fn default() -> Self {
        Action::Lock
    }
}

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) server: String,
    #[serde(skip)]
    pub(crate) action: Action,
    #[serde(skip)]
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
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("generate")
                .about("generate autocomplete for shell")
                .arg(Arg::new("shell").takes_value(true).required(true)),
        )
        .subcommand(App::new("lock").about("Try to reserve a push window"))
        .subcommand(App::new("unlock").about("Release push window"))
        .subcommand(App::new("check").about("Check for current push window"))
        .arg(
            Arg::new("server")
                .long("server")
                .takes_value(true)
                .about("Specify server address"),
        )
}

fn args() -> ArgMatches {
    app().get_matches()
}

impl Config {
    pub(crate) fn from_config() -> Self {
        let mut home_dir = dirs::config_dir().unwrap();
        home_dir.push("pushlock");

        fs::create_dir_all(&home_dir).unwrap();

        home_dir.push("config.toml");

        let f = fs::read_to_string(&home_dir).unwrap_or_else(|_| {
            UserFacingError::new("Unable to open config file")
                .reason("File not found")
                .help(format!(
                    indoc! {"
                    Please create file {} with content:

                    server = <pushlock server>
                    "},
                    home_dir.display()
                ))
                .print_and_exit();
            abort();
        });

        let mut c: Config = toml::from_str(&f).unwrap();
        c.from_args();
        c
    }

    fn from_args(&mut self) {
        let args = args();

        if let Some(generate) = args.subcommand_matches("generate") {
            autocomplete(generate.value_of("shell").unwrap());
            process::exit(0);
        }

        self.action = if args.is_present("lock") {
            Action::Lock
        } else if args.is_present("unlock") {
            Action::Release
        } else if args.is_present("check") {
            Action::CheckState
        } else {
            panic!("Invalid action");
        };

        if let Some(server) = args.value_of("server") {
            self.server = server.to_string();
        }

        self.endpoint = match self.action {
            Action::Lock => format!("http://{}/lock", self.server),
            Action::Release => format!("http://{}/release", self.server),
            Action::CheckState => format!("http://{}/is_locked", self.server),
        };
    }
}
