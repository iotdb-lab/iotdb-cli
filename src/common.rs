use iotdb::{Config, Session};
use log::{error, info};
use simplelog::LevelFilter;
use std::io::BufRead;
use std::path::Path;
use std::{fs, io};

pub const ASCII_NAME: &str = "
▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀\t
";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// Pint slogan
pub fn slogan() -> String {
    format!(
        "{}\nAuthor: {}\nVersion: {} v{}",
        ASCII_NAME, AUTHORS, PKG_NAME, VERSION,
    )
}

/// Exec batch from sql str
pub fn show_exec_sql_from_str(conf: Config, sql: String) -> anyhow::Result<()> {
    let mut session = Session::connect(conf)?;

    let sql_vec: Vec<String> = sql
        .trim()
        .split(';')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .filter(|x| !x.starts_with("--"))
        .map(|x| x.replace('\n', " "))
        .collect();

    if sql_vec.len() > 1 {
        match session.exec_batch(sql_vec.clone()) {
            Ok(_) => info!("SQL: {:#?} Execute batch statements successfully", sql_vec),
            Err(error) => error!("{}", error),
        }
    } else {
        match session.sql(sql_vec[0].as_str()) {
            Ok(mut ds) => ds.show(),
            Err(error) => error!("{}", error),
        }
    }
    session.close()?;
    Ok(())
}

/// Exec batch from sql file
pub fn exec_batch_from_file(conf: Config, file_path: &str) -> anyhow::Result<()> {
    let file = Path::new(&file_path);
    let mut session = Session::connect(conf)?;
    if file.exists() {
        if !file.is_file() || !file_path.ends_with(".sql") {
            error!(" {:?} is not a sql file", file_path);
        } else {
            info!("Statements: {:#?}", sql_file_reader(file_path));
            session.exec_batch(sql_file_reader(file_path))?;
            session.close()?;
        }
    } else {
        error!("{:?} not exist", file_path);
    }
    session.close()?;
    Ok(())
}

/// SQL file reader
pub fn sql_file_reader(file_path: &str) -> Vec<String> {
    let mut batch_sql: Vec<String> = vec![];
    match fs::File::open(file_path) {
        Ok(file) => {
            let sql_lines = io::BufReader::new(file)
                .lines()
                .map(|s| s.unwrap_or_default())
                .filter(|s| !s.is_empty())
                .filter(|s| !s.starts_with("--"))
                .filter(|s| s.len() != 1)
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            let mut tmp_str: String = String::new();
            for line in sql_lines {
                if line.ends_with(';') {
                    if tmp_str.is_empty() {
                        batch_sql.push(line);
                    } else {
                        batch_sql.push(format!("{}{}", tmp_str, line));
                        tmp_str.clear();
                    }
                } else {
                    tmp_str.push_str(line.as_str());
                    tmp_str.push_str("\n ");
                }
            }
            batch_sql
        }
        Err(error) => {
            error!("ERROR: {:?}", error);
            vec![]
        }
    }
}

/// Logger
pub fn logger(level: LevelFilter) {
    use simplelog::*;
    let _ = CombinedLogger::init(vec![TermLogger::new(
        level,
        Default::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )]);
}

/// Print help infos
pub fn print_help() {
    let help_info = include_str!("static/help_info");
    println!("{}", help_info);
}
