use iotdb::{Config, ConfigBuilder, Endpoint, Session};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const ASCII_NAME: &str = "
▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀\t
";

#[derive(StructOpt, Debug)]
#[structopt(name = ASCII_NAME)]
struct Opt {
    /// Execute single sql, eg: `iotdb "show storage group"`
    sql: Option<String>,

    /// Set server hostname or ip address, eg: `127.0.0.1`
    #[structopt(short = "H", long)]
    host: Option<String>,

    /// Set server port
    #[structopt(short = "P", long)]
    port: Option<String>,

    /// Set user name
    #[structopt(short, long)]
    user: Option<String>,

    /// Set user password
    #[structopt(short, long)]
    password: Option<String>,

    /// Set server endpoint, eg: `localhost:6667`
    #[structopt(short, long)]
    endpoint: Option<String>,

    /// Set timezone, eg: `UTC+8`
    #[structopt(short, long)]
    timezone: Option<String>,

    /// Enable debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Subcommands
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Execute batch form sql file, eg: `iotdb file ddl.sql`
    File { path: String },
}

fn main() {
    let mut builder = ConfigBuilder::new();
    // match Opt::from_args() {
    let Opt {
        sql,
        host,
        port,
        user,
        password,
        endpoint,
        timezone,
        debug,
        command,
    } = Opt::from_args();

    // set endpoint
    if let Some(endpoint) = endpoint {
        let endpoint = endpoint.as_str().parse::<Endpoint>().unwrap();
        builder.endpoint(endpoint.host.as_str(), endpoint.port.as_str());
    } else if let Some(host) = host {
        if let Some(port) = port {
            builder.endpoint(host.as_str(), port.as_str());
        }
    }

    // user and password
    if let Some(user) = user {
        if let Some(password) = password {
            builder.user(user.as_str());
            builder.password(password.as_str());
        }
    }

    // timezone
    if let Some(timezone) = timezone {
        builder.time_zone(timezone.as_str());
    }

    // enable debug mode
    builder.debug(debug);

    let prompt = format!("IOTDB#({})> ", builder.build().endpoint.to_string());
    let mut session = open_session(builder.build());

    match command {
        None => {
            if let Some(sql) = sql {
                session.sql(sql.as_str()).unwrap().show()
            } else {
                readline(session, prompt)
            }
        }
        Some(command) => match command {
            Command::File { path } => {
                let sql_file = Path::new(&path);
                if sql_file.exists() {
                    if !sql_file.is_file() || !path.ends_with(".sql") {
                        println!("ERROR: {:?} is not a sql file", sql_file);
                    } else {
                        println!("Statements: {:#?}", sql_file_reader(sql_file));
                        session.exec_batch(sql_file_reader(sql_file));
                    }
                } else {
                    println!("ERROR: {:?} not exist", sql_file);
                }
            }
        },
    }
}

fn open_session(config: Config) -> Session {
    Session::new(config).open().unwrap()
}

fn readline(mut session: Session, prompt: String) {
    println!("{}", ASCII_NAME);
    let his_file: PathBuf = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/home"))
        .join(".iotdb_his");

    let mut rl = Editor::<()>::new();
    if his_file.as_path().exists() {
        rl.load_history(his_file.as_path()).unwrap();
    }
    loop {
        let readline = rl.readline(prompt.as_str());
        match readline {
            Ok(sql) => {
                if sql.trim().is_empty() {
                    continue;
                }

                rl.add_history_entry(sql.as_str());
                if sql.contains("exit") || sql.contains("quit") {
                    session.close().unwrap();
                    break;
                }

                if let Ok(mut ds) = session.sql(sql.as_str()) {
                    ds.show()
                }
            }
            Err(ReadlineError::Interrupted) => {
                session.close().unwrap();
                println!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                session.close().unwrap();
                println!("Ctrl-D");
                break;
            }
            Err(err) => {
                session.close().unwrap();
                println!("Error: {:?}", err);
                break;
            }
        }
        rl.save_history(his_file.as_path()).unwrap();
    }
}

fn sql_file_reader(path: &Path) -> Vec<String> {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let mut batch_sql: Vec<String> = vec![];
    match fs::File::open(path) {
        Ok(file) => {
            let sql_lines = io::BufReader::new(file)
                .lines()
                .map(|s| s.unwrap_or_default())
                .filter(|s| !s.is_empty())
                .filter(|s| !s.starts_with("--"))
                .filter(|s| s.len() != 1)
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            let mut tmp_string: String = String::new();
            let mut is_tmp = false;
            for line in sql_lines {
                if line.ends_with(';') {
                    if is_tmp {
                        tmp_string.push_str(line.as_str());
                        batch_sql.push(tmp_string.clone());
                        tmp_string.clear();
                        is_tmp = false;
                    } else {
                        batch_sql.push(line);
                    }
                } else {
                    tmp_string.push_str(line.as_str());
                    tmp_string.push_str("\n ");
                    is_tmp = true;
                }
            }
            batch_sql
        }
        Err(error) => {
            println!("ERROR: {:?}", error);
            vec![]
        }
    }
}
