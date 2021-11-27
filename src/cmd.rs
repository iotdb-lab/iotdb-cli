use crate::opt::{Cli, SubCmd};
use crate::sub_cmd::{sql_file_reader, update};
use crate::{ASCII_NAME, AUTHORS, PKG_NAME, VERSION};
use iotdb::{Config, ConfigBuilder, Endpoint, Session};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// Run IoTDB CLI
pub fn run() {
    let Cli {
        sql,
        file,
        host,
        port,
        user,
        password,
        endpoint,
        timezone,
        debug,
        sub_cmd,
    } = Cli::new();

    let mut conf_builder = ConfigBuilder::new();
    // set endpoint
    if let Some(endpoint) = endpoint {
        let endpoint = endpoint.as_str().parse::<Endpoint>().unwrap();
        conf_builder.endpoint(endpoint.host.as_str(), endpoint.port.as_str());
    } else if let Some(host) = host {
        if let Some(port) = port {
            conf_builder.endpoint(host.as_str(), port.as_str());
        }
    }

    // user and password
    if let Some(user) = user {
        if let Some(password) = password {
            conf_builder.user(user.as_str());
            conf_builder.password(password.as_str());
        }
    }

    // timezone
    if let Some(timezone) = timezone {
        conf_builder.time_zone(timezone.as_str());
    }

    // enable debug mode
    conf_builder.debug(debug);
    let conf = conf_builder.build();

    // exec batch
    if let Some(file_path) = file {
        exec_batch_from_file(conf, file_path.as_str());
        std::process::exit(0);
    }

    // open session
    let prompt = format!("IOTDB#({})> ", conf.endpoint.to_string());
    let mut session = open_session(conf.clone());

    // sub command
    match sub_cmd {
        None => {
            if let Some(sql) = sql {
                session.sql(sql.as_str()).unwrap().show()
            } else {
                readline(session, prompt)
            }
        }
        Some(sub_cmd) => match sub_cmd {
            // exec batch
            SubCmd::File { file_path } => {
                if let Some(file_path) = file_path {
                    exec_batch_from_file(conf, &file_path);
                }
            }
            SubCmd::Usage => {
                print_usage_info();
            }
            SubCmd::Update => {
                update();
            }
        },
    }
}

/// Open a IoTDB session
fn open_session(config: Config) -> Session {
    Session::new(config).open().unwrap()
}

/// readline
fn readline(mut session: Session, prompt: String) {
    let fore_space = "    ";
    println!(
        "{}\nAuthor: {}\nVersion: {} v{}\nUsage:\n{}\
        1. Print usage info: `?` or `help` \n{}\
        2. Exec system command on OS: `!ps`\n{}\
        3. Exit: `exit` or `quit` or `Ctrl-C` or `Ctrl-D`",
        ASCII_NAME, AUTHORS, PKG_NAME, VERSION, fore_space, fore_space, fore_space
    );
    let his_file: PathBuf = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/home"))
        .join(".iotdb_his");

    let mut rl = Editor::<()>::new();
    if his_file.as_path().exists() {
        rl.load_history(his_file.as_path()).unwrap();
    }

    let mut tmp_sql: String = String::new();
    loop {
        // TODO: is_open is invalid and needs to be fixed in iotdb-rs
        if session.is_close() {
            session = session.open().unwrap();
        }
        let readline;
        if !tmp_sql.is_empty() {
            readline = rl.readline(">> ");
        } else {
            readline = rl.readline(prompt.as_str());
        }

        match readline {
            Ok(mut sql) => {
                if sql.contains("exit") || sql.contains("quit") {
                    session.close().unwrap();
                    rl.add_history_entry(sql.as_str());
                    rl.save_history(his_file.as_path()).unwrap();
                    break;
                }

                if sql.trim().is_empty() {
                    continue;
                }

                if sql.eq("?") || sql.eq("help") {
                    rl.add_history_entry(sql.as_str());
                    print_usage_info();
                    continue;
                }

                if sql.starts_with('!') {
                    rl.add_history_entry(sql.as_str());
                    sql.remove(0);
                    exec_shell_cmd(sql);
                    continue;
                }

                if sql.ends_with(';') {
                    if tmp_sql.is_empty() {
                        if let Ok(mut ds) = session.sql(sql.as_str()) {
                            ds.show()
                        }
                    } else {
                        sql = format!("{}{}", tmp_sql, sql);
                        println!("```sql\n{}\n```", sql);
                        if let Ok(mut ds) = session.sql(sql.as_str()) {
                            ds.show()
                        }
                        tmp_sql.clear();
                    }
                    rl.add_history_entry(sql.as_str());
                } else {
                    tmp_sql.push_str(sql.trim());
                    tmp_sql.push('\n')
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

// Exec batch from sql file
fn exec_batch_from_file(conf: Config, file_path: &str) {
    let file = Path::new(&file_path);
    let mut session = open_session(conf);
    if file.exists() {
        if !file.is_file() || !file_path.ends_with(".sql") {
            println!("ERROR: {:?} is not a sql file", file_path);
        } else {
            println!("Statements: {:#?}", sql_file_reader(file_path));
            session.exec_batch(sql_file_reader(file_path));
            session.close().unwrap();
        }
    } else {
        println!("ERROR: {:?} not exist", file_path);
    }
}

/// Print help info
fn print_usage_info() {
    let help_info = include_str!("static/usage_info");
    println!("{}", help_info);
}

/// Exec shell command
#[allow(unused_assignments)]
fn exec_shell_cmd(cmd_str: String) {
    let mut cmd_str = cmd_str;
    if cfg!(target_os = "windows") {
        cmd_str = "dir c:\\tmp".to_string();
    }

    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .arg("/c")
            .arg(cmd_str)
            .stdout(Stdio::piped())
            .output()
            .expect("cmd exec error!");
        output_to_stdout(output);
    } else {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .stdout(Stdio::piped())
            .output()
            .expect("sh exec error!");
        output_to_stdout(output);
    };
}

/// Command output to stdout
fn output_to_stdout(output: Output) {
    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        io::stdout().write_all(&output.stderr).unwrap();
    }
}
