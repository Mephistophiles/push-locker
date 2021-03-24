use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn colored<F>(color: Color, f: F)
where
    F: FnOnce(),
{
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .unwrap();

    f();

    stdout.reset().unwrap();
}

pub fn red<F: FnOnce()>(f: F) {
    colored(Color::Red, f)
}

pub fn green<F: FnOnce()>(f: F) {
    colored(Color::Green, f)
}

pub fn yellow<F: FnOnce()>(f: F) {
    colored(Color::Yellow, f)
}
