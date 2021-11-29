use crate::VERSION;
use std::fs;
use std::io;
use std::io::BufRead;

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

#[allow(dead_code)]
pub fn update() {
    println!("Update to {:?}", VERSION)
}

#[allow(dead_code)]
pub fn download_latest() {
    todo!()
}

#[allow(dead_code)]
pub fn replace_binary() {
    todo!()
}
