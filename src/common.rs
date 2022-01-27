use iotdb::{Config, Session};
use simplelog::LevelFilter;
use std::io::BufRead;
use std::path::Path;
use std::{fs, io};

// Exec batch from sql file
pub fn exec_batch_from_file(conf: Config, file_path: &str) -> anyhow::Result<()> {
    let file = Path::new(&file_path);
    let mut session = Session::connect(conf)?;
    if file.exists() {
        if !file.is_file() || !file_path.ends_with(".sql") {
            println!("ERROR: {:?} is not a sql file", file_path);
        } else {
            println!("Statements: {:#?}", sql_file_reader(file_path));
            session.exec_batch(sql_file_reader(file_path))?;
            session.close()?;
        }
    } else {
        println!("ERROR: {:?} not exist", file_path);
    }
    Ok(())
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
            println!("ERROR: {:?}", error);
            vec![]
        }
    }
}

pub fn print_help() {
    let help_info = include_str!("static/help_info");
    println!("{}", help_info);
}
