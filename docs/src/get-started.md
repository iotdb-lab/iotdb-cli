<div align="center">

![Logo](https://raw.githubusercontent.com/iotdb-lab/iotdb-rs/main/iotdb-rs.png)

<h1>iotdb-cli</h1>
<h3>Apache IotDB CLI Client written in Rust</h3>

[![Contribute](https://img.shields.io/badge/contribute-now-a94064?color=%23E5531A&)](https://gitpod.io/#https://github.com/iotdb-lab/iotdb-cli)
[![downloads](https://img.shields.io/crates/d/iotdb-cli?style=flat-square&color=%23E5531A)](https://crates.io/crates/iotdb-cli)
[![GitHub Release](https://img.shields.io/github/v/release/iotdb-lab/iotdb-cli?include_prereleases&sort=semver&color=%23E5531A&style=flat-square)](https://github.com/iotdb-lab/iotdb-cli/releases)
![Top Lang](https://img.shields.io/github/languages/top/trisasnava/koifish?color=%23E5531A&style=flat-square)
[![Rust Build](https://img.shields.io/github/workflow/status/iotdb-lab/iotdb-cli/cargo-test?label=build&style=flat-square)](https://github.com/iotdb-lab/iotdb-cli/actions?query=workflow%3Acargo-test)
[![Docs Build](https://img.shields.io/github/workflow/status/iotdb-lab/iotdb-cli/mdbook-deploy?label=docs%20build&style=flat-square)](https://github.com/iotdb-lab/iotdb-cli/actions?query=workflow:mdbook-deploy)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square&color=%23E5531A)](https://github.com/iotdb-lab/iotdb-cli/blob/main/LICENSE)
[![Crates Publish](https://img.shields.io/github/workflow/status/iotdb-lab/iotdb-cli/cargo-publish?label=publish&style=flat-square)](https://github.com/iotdb-lab/iotdb-cli/actions?query=workflow%3Acargo-publish)

</div>

---

[![Alt](https://repobeats.axiom.co/api/embed/86055cf67fcaac9e6e93c64c9a7a1630686ceda1.svg "Repobeats analytics image")](https://github.com/iotdb-lab/iotdb-cli/pulse)

## Docker

```shell
docker run --name iotdb-cli -d ghcr.io/iotdb-lab/iotdb-cli
docker exec -it iotdb-cli iotdb
```

## Installation

[![Linux supported](https://img.shields.io/badge/Linux%20x86__64-supported%20✓-228B22?style=flat-square&logo=linux)](https://github.com/iotdb-lab/iotdb-cli/releases/latest)
[![macOS supported](https://img.shields.io/badge/macOS%20x86__64-supported%20✓-228B22?style=flat-square&logo=apple)](https://github.com/iotdb-lab/iotdb-cli/releases/latest)
[![Windows supported](https://img.shields.io/badge/Windows%20x86__64-supported%20✓-228B22?style=flat-square&logo=windows)](https://github.com/iotdb-lab/iotdb-cli/releases/latest)

1. Using `Cargo`

```shell
cargo install -f iotdb-cli

cargo install -f --git  https://github.com/iotdb-lab/iotdb-cli.git
```

2. From [binary](https://github.com/iotdb-lab/iotdb-cli/releases/latest)

```shell
curl -s https://raw.githubusercontent.com/iotdb-lab/iotdb-cli/main/install.sh | bash

curl -s https://raw.githubusercontent.com/iotdb-lab/iotdb-cli/main/install.sh | bash -s -- x.x.x
```

```shell
wget -qO- https://raw.githubusercontent.com/iotdb-lab/iotdb-cli/main/install.sh | bash

wget -qO- https://raw.githubusercontent.com/iotdb-lab/iotdb-cli/main/install.sh | bash -s -- x.x.x
```

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
Version: iotdb-cli 0.0.3-alpha

USAGE:
    iotdb [FLAGS] [OPTIONS] [sql] [SUBCOMMAND]

FLAGS:
    -d, --debug      Enable debug mode
        --dev        Connect to dev server
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
    csv        Csv util(TODO)
    file       Execute batch form sql file, eg: `iotdb file ddl.sql`
    help       Prints this message or the help of the given subcommand(s)
    load       Load TsFile util (TODO)
    update     Self update(TODO)
    usage      Print usage
    version    Prints server version info
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

Author: github.com/iotdb-lab <me@francis.run>
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