use crate::common;
use crate::opt::{Cli, SubCmd};
use crate::slogan;
use iotdb::{Config, ConfigBuilder, Endpoint, Session};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use simplelog::LevelFilter;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        let Cli {
            sql, file, sub_cmd, ..
        } = self;

        let conf = self.session_conf();

        // exec batch
        if let Some(file_path) = file {
            common::exec_batch_from_file(conf, file_path.as_str())?;
            std::process::exit(0);
        }

        // sub command
        match sub_cmd {
            None => {
                // open session
                let prompt = format!("IOTDB#({})> ", conf.endpoint.to_string());
                let mut session = Session::connect(conf)?;

                if let Some(sql) = sql {
                    session.sql(sql.as_str())?.show()
                } else {
                    self.readline(session, prompt)
                }
            }
            Some(sub_cmd) => {
                match sub_cmd {
                    // exec sql form file
                    SubCmd::File { file_path } => {
                        if let Some(file) = file_path {
                            common::exec_batch_from_file(conf, file)?;
                        }
                    }
                    SubCmd::Usage => sub_cmd.help(),
                    SubCmd::Update => sub_cmd.update(),
                    SubCmd::Csv { .. } => {}
                    SubCmd::Load => {}
                    SubCmd::Version => {
                        println!("{}", slogan());
                        let mut session = Session::connect(self.session_conf())?;
                        session.sql("show version")?.show();
                    }
                }
            }
        }

        Ok(())
    }

    /// Set session conf
    pub fn session_conf(&self) -> Config {
        let Cli {
            dev,
            host,
            port,
            user,
            password,
            endpoint,
            timezone,
            debug,
            ..
        } = self;

        let mut builder = ConfigBuilder::new();

        // timezone
        if let Some(timezone) = timezone {
            builder.time_zone(timezone.as_str());
        }

        // enable debug mode
        if *debug {
            common::logger(LevelFilter::Debug);
        } else {
            common::logger(LevelFilter::Info);
        }

        // user and password
        if let Some(user) = user {
            builder.user(user.as_str());
        }
        if let Some(password) = password {
            builder.password(password.as_str());
        }

        // set endpoint
        if host.is_some() && port.is_some() {
            builder.host_port(
                host.as_ref().unwrap().as_str(),
                port.as_ref().unwrap().as_str(),
            );
        } else if let Some(endpoint) = endpoint {
            let endpoint = endpoint.as_str().parse::<Endpoint>().unwrap();
            builder.endpoint(endpoint.host.as_str());
        } else if *dev {
            builder.endpoint("119.84.128.59:6667");
        }

        builder.build()
    }

    /// Exec shell command
    fn exec_shell_cmd(&self, cmd_str: String) {
        let mut cmd_str = cmd_str;
        if cfg!(target_os = "windows") {
            cmd_str = "dir C:\\tmp".to_string();
        }

        if cfg!(target_os = "windows") {
            let output = Command::new("cmd")
                .arg("/c")
                .arg(cmd_str)
                .stdout(Stdio::piped())
                .output()
                .expect("cmd exec error!");
            self.output_to_stdout(output);
        } else {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd_str)
                .stdout(Stdio::piped())
                .output()
                .expect("sh exec error!");
            self.output_to_stdout(output);
        };
    }

    /// Command output to stdout
    fn output_to_stdout(&self, output: Output) {
        if output.status.success() {
            io::stdout().write_all(&output.stdout).unwrap();
        } else {
            io::stdout().write_all(&output.stderr).unwrap();
        }
    }

    /// Usage info
    fn cli_usage(&self) -> String {
        let fore_space = "    ";
        format!(
            "Usage:\n{}\
        1. Print help info: `?` or `help` \n{}\
        2. Exec system command on local machine, eg: `!ps -ef`\n{}\
        3. Exit: `exit` or `quit` or `Ctrl-C` or `Ctrl-D`\n",
            fore_space, fore_space, fore_space
        )
    }

    /// Print help info
    pub fn help(&self) {
        common::print_help();
    }

    /// readline
    fn readline(&self, mut session: Session, prompt: String) {
        println!("{}\n{}", slogan(), self.cli_usage());
        let his_file: PathBuf = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/home"))
            .join(".iotdb_his");

        let mut rl = Editor::<()>::new();
        if his_file.as_path().exists() {
            rl.load_history(his_file.as_path()).unwrap();
        }

        let mut tmp_sql: String = String::new();
        let mut max_str_len = 0;
        loop {
            // TODO: is_open is invalid and needs to be fixed in iotdb-rs
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
                        self.help();
                        continue;
                    }

                    if sql.starts_with('!') {
                        rl.add_history_entry(sql.as_str());
                        sql.remove(0);
                        self.exec_shell_cmd(sql);
                        continue;
                    }

                    if sql.ends_with(';') {
                        if tmp_sql.is_empty() {
                            if let Ok(mut ds) = session.sql(sql.as_str()) {
                                ds.show()
                            }
                        } else {
                            sql = format!("{}{}", tmp_sql, sql);

                            let mut split_line = String::new();
                            for _i in 0..max_str_len {
                                split_line.push('+')
                            }
                            println!("{}\n{}\n{}", split_line, sql, split_line);

                            if let Ok(mut ds) = session.sql(sql.as_str()) {
                                ds.show()
                            }

                            tmp_sql.clear();
                            max_str_len = 0;
                        }
                        rl.add_history_entry(sql.as_str());
                    } else {
                        tmp_sql.push_str(sql.trim());
                        tmp_sql.push('\n');

                        if sql.len() > max_str_len {
                            max_str_len = sql.len();
                        }
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
}
