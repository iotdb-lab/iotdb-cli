use crate::common::{ASCII_NAME, AUTHORS, PKG_NAME};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = format!("{}\nAuthor: {}\nVersion: {}", ASCII_NAME, AUTHORS, PKG_NAME))]
pub struct Cli {
    /// Execute single sql, eg: `iotdb "show storage group"`
    pub sql: Option<String>,

    /// Connect to dev server
    #[structopt(long)]
    pub dev: bool,

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

impl Cli {
    pub fn new() -> Self {
        Cli::from_args()
    }
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = format!("{}\nAuthor: {}\nVersion: {}", ASCII_NAME, AUTHORS, PKG_NAME))]
pub enum SubCmd {
    /// Execute batch form sql file, eg: `iotdb file ddl.sql`
    File { file_path: Option<String> },

    /// Csv util(TODO)
    Csv {
        #[structopt(short, long)]
        import: Option<String>,

        #[structopt(short, long)]
        export: Option<String>,
    },

    /// Load TsFile util (TODO)
    Load,

    /// Print usage
    Usage,

    /// Self update(TODO)
    Update,

    /// Prints server version info
    Version,
}
