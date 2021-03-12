use clap::{App, Arg};

pub(crate) fn get_port() -> u16 {
    let app = App::new("pushlock-server")
        .arg(
            Arg::new("port")
                .takes_value(true)
                .required(true)
                .default_value("8080"),
        )
        .get_matches();

    app.value_of("port")
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}
