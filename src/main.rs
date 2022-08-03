use crate::common::*;
use crate::opt::Cli;

mod cmd;
mod common;
mod opt;
mod sub_cmd;

/// Run IoTDB CLI
fn main() -> anyhow::Result<()> {
    Cli::new().run()?;
    Ok(())
}

pub fn slogan() -> String {
    format!(
        "{}\nAuthor: {}\nVersion: {} v{}",
        ASCII_NAME, AUTHORS, PKG_NAME, VERSION,
    )
}
