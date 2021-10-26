use iotdb::{Config, Endpoint, Session};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::PathBuf;
use structopt::StructOpt;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const ASCII_NAME: &str = "\
▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀\t
";

#[derive(StructOpt, Debug)]
#[structopt(name = ASCII_NAME)]
struct CliOpts {
    /// Server host name
    #[structopt(short = "H", long)]
    host: Option<String>,

    /// Server port
    #[structopt(short = "P", long)]
    port: Option<String>,

    /// User name
    #[structopt(short, long)]
    user: Option<String>,

    /// User password
    #[structopt(short, long)]
    password: Option<String>,

    /// Endpoint
    #[structopt(long)]
    endpoint: Option<String>,

    /// Logger level
    #[structopt(long)]
    log_level: Option<String>,

    /// Enable debug mode
    #[structopt(short, long)]
    debug: bool,
}

fn readline(config: Config) {
    let his_file: PathBuf = dirs::home_dir()
        .unwrap_or(PathBuf::from("/home"))
        .join("iotdb.his");
    let session = Session::new(config.clone());

    match session.open() {
        Ok(mut session) => {
            println!(
                "{}\nConnect server: {}\nVersion: {}",
                ASCII_NAME,
                config.endpoint.to_string(),
                VERSION
            );

            let mut rl = Editor::<()>::new();
            rl.load_history(his_file.as_path()).unwrap();
            loop {
                let readline = rl.readline("IOTDB#> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        if line.contains("exit") || line.contains("quit") {
                            break;
                        }
                        match session.sql(line.as_str()) {
                            Ok(mut ds) => ds.show(),
                            Err(_) => {}
                        }
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
            rl.save_history(his_file.as_path()).unwrap();
        }
        Err(error) => panic!("{}", error),
    }
}

fn main() {
    let mut config = Config::new();
    match CliOpts::from_args() {
        CliOpts {
            host,
            port,
            user,
            password,
            endpoint,
            log_level,
            debug,
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

            // log level
            if log_level.is_some() {
                config.log_level(log_level.unwrap().as_str());
            }
            config.debug(debug).build();
        }
    }

    readline(config)
}
