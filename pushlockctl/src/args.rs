use clap::{Arg, ArgMatches, Command};
use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
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

fn default_username() -> String {
    whoami::username()
}

#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde(default = "default_username")]
    pub(crate) username: String,

    pub(crate) server: String,
    #[serde(skip)]
    pub(crate) action: Action,
    #[serde(skip)]
    pub(crate) endpoint: String,
}

fn autocomplete(shell: &str) {
    let mut app = app();

    match shell {
        "bash" => generate(Bash, &mut app, clap::crate_name!(), &mut io::stdout()),
        "elvish" => generate(Elvish, &mut app, clap::crate_name!(), &mut io::stdout()),
        "fish" => generate(Fish, &mut app, clap::crate_name!(), &mut io::stdout()),
        "powershell" => generate(PowerShell, &mut app, clap::crate_name!(), &mut io::stdout()),
        "zsh" => generate(Zsh, &mut app, clap::crate_name!(), &mut io::stdout()),
        _ => panic!("Unknown generator"),
    }
}

fn app() -> Command<'static> {
    Command::new("pushlock")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .infer_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .override_help("generate autocomplete for shell")
                .arg(Arg::new("shell").takes_value(true).required(true)),
        )
        .subcommand(Command::new("lock").override_help("Try to reserve a push window"))
        .subcommand(Command::new("unlock").override_help("Release push window"))
        .subcommand(Command::new("check").override_help("Check for current push window"))
        .arg(
            Arg::new("username")
                .long("username")
                .takes_value(true)
                .help("Specify username"),
        )
        .arg(
            Arg::new("server")
                .long("server")
                .takes_value(true)
                .help("Specify server address"),
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

                    server = \"<pushlock server ip>:<pushlock server port>\"
                    username = \"<uniq username>\"
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

        if let Some(username) = args.value_of("username") {
            self.username = username.to_string();
        }

        self.endpoint = match self.action {
            Action::Lock => format!("http://{}/lock", self.server),
            Action::Release => format!("http://{}/release", self.server),
            Action::CheckState => format!("http://{}/lock_state", self.server),
        };
    }
}
