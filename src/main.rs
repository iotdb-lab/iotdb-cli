use crate::opt::Cli;

mod cmd;
mod common;
mod opt;
mod sub_cmd;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const ASCII_NAME: &str = "
▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀\t
";

/// Run IoTDB CLI
fn main() {
    Cli::new().run();
}

pub fn slogan() -> String {
    format!(
        "{}\nAuthor: {}\nVersion: {} v{}",
        ASCII_NAME, AUTHORS, PKG_NAME, VERSION,
    )
}
