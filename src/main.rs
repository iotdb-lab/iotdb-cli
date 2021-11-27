mod cmd;
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

fn main() {
    cmd::run();
}
