use crate::{ASCII_NAME, AUTHORS, PKG_NAME};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = format ! ("{}\nAuthor: {}\nVersion: {}", ASCII_NAME, AUTHORS, PKG_NAME))]
pub struct Cli {
    /// Execute single sql, eg: `iotdb "show storage group"`
    pub sql: Option<String>,

    /// Execute batch form sql file, eg: `iotdb -f ddl.sql`
    #[structopt(short = "f", long)]
    pub file: Option<String>,

    /// Set server hostname or ip address, eg: `127.0.0.1`
    #[structopt(short = "H", long)]
    pub host: Option<String>,

    /// Set server port
    #[structopt(short = "P", long)]
    pub port: Option<String>,

    /// Set user name
    #[structopt(short, long)]
    pub user: Option<String>,

    /// Set user password
    #[structopt(short, long)]
    pub password: Option<String>,

    /// Set server endpoint, eg: `localhost:6667`
    #[structopt(short, long)]
    pub endpoint: Option<String>,

    /// Set timezone, eg: `UTC+8`
    #[structopt(short, long)]
    pub timezone: Option<String>,

    /// Enable debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Subcommands
    #[structopt(subcommand)]
    pub sub_cmd: Option<SubCmd>,
}

#[derive(Debug, StructOpt)]
pub enum SubCmd {
    /// Execute batch form sql file, eg: `iotdb file ddl.sql`
    File { file_path: Option<String> },

    /// Print usage info
    Usage,

    /// Update binary(TODO)
    Update,
}

impl Cli {
    pub fn new() -> Self {
        Cli::from_args()
    }
}