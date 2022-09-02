use crate::api;
use crate::model;
use std::io::Write;
use termcolor::WriteColor;

pub fn write_color(text: &str, color: termcolor::Color) -> std::io::Result<()> {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(color)))?;
    write!(&mut stdout, "{}", text)
}

pub fn display(
    body: &String,
    case: u8,
    index: usize,
    api: &model::Api,
    is_cache: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match api {
        model::Api::Oxford => api::oxford::display::display(body, case, index, is_cache),
        _ => api::free::display::display(body, case, index, is_cache),
    }
}
