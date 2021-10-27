use iotdb::{Config, Endpoint, Session};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::PathBuf;
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
    /// Execute sql like `iotdb "SHOW STORAGE GROUP"`
    sql: Option<String>,

    /// Set server hostname or IP
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

    /// Set server endpoint, eg: host:port
    #[structopt(long)]
    endpoint: Option<String>,

    /// Set timezone, eg: UTC+8
    #[structopt(short, long)]
    timezone: Option<String>,

    /// Set logger level
    #[structopt(long)]
    log_level: Option<String>,

    /// Enable debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Subcommands
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    ///TODO: Execute sql from file
    File { path: String },
}

fn main() {
    let mut config = Config::new();
    match Opt::from_args() {
        Opt {
            sql,
            host,
            port,
            user,
            password,
            endpoint,
            timezone,
            log_level,
            debug,
            command,
        } => {
            // set endpoint
            if host.is_some() && port.is_some() {
                config.endpoint(host.unwrap().as_str(), port.unwrap().as_str());
            }

            if endpoint.is_some() {
                let endpoint = endpoint.unwrap().as_str().parse::<Endpoint>().unwrap();
                config.endpoint(endpoint.host.as_str(), endpoint.port.as_str());
            }

            // user and password
            if user.is_some() && password.is_some() {
                config.user(user.unwrap().as_str());
                config.password(password.unwrap().as_str());
            }

            // timezone
            if timezone.is_some() {
                config.zone_id(timezone.unwrap().as_str());
            }

            // log level
            if log_level.is_some() {
                config.log_level(log_level.unwrap().as_str());
            }
            config.debug(debug).build();

            let prompt = format!("IOTDB#({})> ", config.clone().endpoint.to_string());
            let mut session = open_session(config);

            match command {
                None => {
                    if sql.is_none() {
                        readline(session, prompt)
                    } else {
                        session.sql(sql.unwrap().as_str()).unwrap().show()
                    }
                }
                Some(command) => match command {
                    Command::File { .. } => todo!(),
                },
            }
        }
    }
}

fn open_session(config: Config) -> Session {
    Session::new(config.clone()).open().unwrap()
}

fn readline(mut session: Session, prompt: String) {
    println!("{}", ASCII_NAME);

    let his_file: PathBuf = dirs::home_dir()
        .unwrap_or(PathBuf::from("/home"))
        .join("iotdb.his");

    let mut rl = Editor::<()>::new();
    rl.load_history(his_file.as_path()).unwrap();
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

                match session.sql(sql.as_str()) {
                    Ok(mut ds) => ds.show(),
                    Err(_) => {}
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
