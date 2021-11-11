<div align="center">

![Logo](https://raw.githubusercontent.com/francis-du/iotdb-rs/main/iotdb-rs.png)

<h1>iotdb-cli</h1>
<h3>Apache IotDB CLI Client written in Rust</h3>

[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square&color=%23E5531A)](https://github.com/francis-du/iotdb-cli/blob/main/LICENSE)
[![Rust Build](https://img.shields.io/github/workflow/status/francis-du/iotdb-cli/cargo-test?label=build&style=flat-square)](https://github.com/francis-du/iotdb-cli/actions?query=workflow%3Acargo-test)
[![Crates Publish](https://img.shields.io/github/workflow/status/francis-du/iotdb-cli/cargo-publish?label=publish&style=flat-square)](https://github.com/francis-du/iotdb-cli/actions?query=workflow%3Acargo-publish)

</div>

---

![Alt](https://repobeats.axiom.co/api/embed/86055cf67fcaac9e6e93c64c9a7a1630686ceda1.svg "Repobeats analytics image")

## Installation

1. Using `Cargo`

```shell
cargo install iotdb-cli
```

2. From binary

Download latest `iotdb` binary from [here](https://github.com/francis-du/iotdb-cli/releases/latest/).

## Usage

```shell
iotdb -h
```

```shell

▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀      0.0.1

USAGE:
    iotdb [FLAGS] [OPTIONS] [sql] [SUBCOMMAND]

FLAGS:
    -d, --debug      Enable debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --endpoint <endpoint>      Set server endpoint, eg: host:port
    -H, --host <host>              Set server hostname or IP
        --log-level <log-level>    Set logger level
    -p, --password <password>      Set user password
    -P, --port <port>              Set server port
    -t, --timezone <timezone>      Set timezone, eg: UTC+8
    -u, --user <user>              Set user name

ARGS:
    <sql>    Execute sql like `iotdb "SHOW STORAGE GROUP"`

SUBCOMMANDS:
    file    TODO: Execute sql from file
    help    Prints this message or the help of the given subcommand(s)

```

1. Connect to server

```shell
iotdb -u root -p root --endpoint 127.0.0.1:6667 -t UTC+8
```

2. Exec SQL

```shell
$ iotdb "SHOW STORAGE GROUP"
+---------------+
| storage group |
+---------------+
| root.ln       |
| root.sg1      |
+---------------+
```

```shell
$ iotdb -u root -p root --endpoint 127.0.0.1:6667 -t UTC+8

▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀     

IOTDB#(127.0.0.1:6667)>  SHOW STORAGE GROUP
+---------------+
| storage group |
+---------------+
| root.ln       |
| root.sg1      |
+---------------+
```

# License

[Apache License 2.0](LICENSE)