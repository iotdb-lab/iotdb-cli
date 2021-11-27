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
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀     

Author: github.com/francis-du <me@francis.run>
Version: iotdb-cli 0.0.2

USAGE:
    iotdb [FLAGS] [OPTIONS] [sql] [SUBCOMMAND]

FLAGS:
    -d, --debug      Enable debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --endpoint <endpoint>    Set server endpoint, eg: `localhost:6667`
    -f, --file <file>            Execute batch form sql file, eg: `iotdb -f ddl.sql`
    -H, --host <host>            Set server hostname or ip address, eg: `127.0.0.1`
    -p, --password <password>    Set user password
    -P, --port <port>            Set server port
    -t, --timezone <timezone>    Set timezone, eg: `UTC+8`
    -u, --user <user>            Set user name

ARGS:
    <sql>    Execute single sql, eg: `iotdb "show storage group"`

SUBCOMMANDS:
    file      Execute batch form sql file, eg: `iotdb file ddl.sql`
    help      Prints this message or the help of the given subcommand(s)
    update    Update binary(TODO)
    usage     Print usage info

```

1. Connect to IoTDB server

- Use default username and password

```shell
$ iotdb "SHOW STORAGE GROUP"
+---------------+
| storage group |
+---------------+
| root.ln       |
| root.sg1      |
+---------------+
```

- Specify parameters

```shell
iotdb -u root -p root -e 127.0.0.1:6667 -t UTC+8

or 

iotdb -u root -p root -H 127.0.0.1 -P 6667 -t UTC+8
```

2. Execute single SQL interactively

```shell
$ iotdb -u root -p root --e 127.0.0.1:6667 -t UTC+8

▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀     

Author: github.com/francis-du <me@francis.run>
Version: iotdb-cli v0.0.2
Usage:
    1. Print usage info: `?` or `help` 
    2. Exec system command on OS: `!ps`
    3. Exit: `exit` or `quit` or `Ctrl-C` or `Ctrl-D`
IOTDB#(127.0.0.1:6667)>  SHOW STORAGE GROUP
+---------------+
| storage group |
+---------------+
| root.ln       |
| root.sg1      |
+---------------+
```

3. Execute sql from the specified sql file

```shell
$ iotdb file tests/create_and_insert.sql
$ iotdb -file tests/create_and_insert.sql
$ iotdb -f tests/create_and_insert.sql
Statements: [
    "DELETE STORAGE GROUP root.test;",
    "CREATE TIMESERIES root.test.status WITH DATATYPE=BOOLEAN, ENCODING=PLAIN;",
    "CREATE TIMESERIES root.test.temperature WITH DATATYPE=FLOAT, ENCODING=RLE;",
    "INSERT INTO root.test(timestamp, status)\n values (1637960249484, true);",
    "INSERT INTO root.test(timestamp, status, temperature)\n values (1637960256493, false, 20.71);",
    "INSERT INTO root.test(timestamp, status, temperature)\n values (1637960261494, true, 32.43);",
    "INSERT INTO root.test(timestamp, status, temperature)\n values (1637960272492, false, 28.66);",
    "INSERT INTO root.test(timestamp, status, temperature)\n values (1637960272492, true, 22.61);",
    "INSERT INTO root.test(timestamp, status, temperature)\n values (1637960296493, false, 28.66);",
]
23:00:23 [INFO] Execute batch statements successfully
```

4. Print usage info

```shell
$ iotdb usage
```

# License

[Apache License 2.0](LICENSE)
