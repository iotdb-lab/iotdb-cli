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
