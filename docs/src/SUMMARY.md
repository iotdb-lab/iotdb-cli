<div align="center">

![Logo](https://raw.githubusercontent.com/francis-du/iotdb-rs/main/iotdb-rs.png)

<h1>iotdb-cli</h1>
<h3>(WIP) Apache IotDB CLI Client written in Rust</h3>

[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square&color=%23E5531A)](https://github.com/francis-du/iotdb-cli/blob/main/LICENSE)
[![Rust Build](https://img.shields.io/github/workflow/status/francis-du/iotdb-cli/cargo-test?label=build&style=flat-square)](https://github.com/francis-du/iotdb-cli/actions?query=workflow%3Acargo-test)
[![Crates Publish](https://img.shields.io/github/workflow/status/francis-du/iotdb-cli/cargo-publish?label=publish&style=flat-square)](https://github.com/francis-du/iotdb-cli/actions?query=workflow%3Acargo-publish)

</div>

---

## Installation

```shell
cargo install iotdb-cli
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
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀      0.0.1

USAGE:
    iotdb-cli [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Enable debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --endpoint <endpoint>      Endpoint
    -H, --host <host>              Server host name
        --log-level <log-level>    Logger level
    -p, --password <password>      User password
    -P, --port <port>              Server port
    -u, --user <user>              User name
```

## Run

1. Connect to IotDB server

```shell
iotdb -u root -p root --endpoint 127.0.0.1:6667 -t UTC+8
```

2. Exec SQL

```sql
SET STORAGE GROUP TO root.ln
SHOW STORAGE GROUP

CREATE TIMESERIES root.ln.wf01.wt01.status WITH DATATYPE=BOOLEAN, ENCODING=PLAIN
CREATE TIMESERIES root.ln.wf01.wt01.temperature WITH DATATYPE=FLOAT, ENCODING=RLE

SHOW TIMESERIES
SHOW TIMESERIES root.ln.wf01.wt01.status

INSERT INTO root.ln.wf01.wt01(timestamp,status) values(100,true);
INSERT INTO root.ln.wf01.wt01(timestamp,status,temperature) values(200,false,20.71)

SELECT status FROM root.ln.wf01.wt01
SELECT * FROM root.ln.wf01.wt01

```

3. Result

```shell
▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀     

Connect server: 127.0.0.1:6667
Version: 0.0.1
IOTDB#> SET STORAGE GROUP TO root.ln
IOTDB#> SHOW STORAGE GROUP
+---------------+
| storage group |
+===============+
| root.ln       |
+---------------+
| root.sg1      |
+---------------+
IOTDB#> 
IOTDB#> CREATE TIMESERIES root.ln.wf01.wt01.status WITH DATATYPE=BOOLEAN, ENCODING=PLAIN
IOTDB#> CREATE TIMESERIES root.ln.wf01.wt01.temperature WITH DATATYPE=FLOAT, ENCODING=RLE
IOTDB#> 
IOTDB#> SHOW TIMESERIES
+-------------------------------+-------+---------------+----------+----------+-------------+------+------------+
| timeseries                    | alias | storage group | dataType | encoding | compression | tags | attributes |
+===============================+=======+===============+==========+==========+=============+======+============+
| root.ln.wf01.wt01.temperature | null  | root.ln       | FLOAT    | RLE      | SNAPPY      | null | null       |
+-------------------------------+-------+---------------+----------+----------+-------------+------+------------+
| root.ln.wf01.wt01.status      | null  | root.ln       | BOOLEAN  | PLAIN    | SNAPPY      | null | null       |
+-------------------------------+-------+---------------+----------+----------+-------------+------+------------+
IOTDB#> SHOW TIMESERIES root.ln.wf01.wt01.status
+--------------------------+-------+---------------+----------+----------+-------------+------+------------+
| timeseries               | alias | storage group | dataType | encoding | compression | tags | attributes |
+==========================+=======+===============+==========+==========+=============+======+============+
| root.ln.wf01.wt01.status | null  | root.ln       | BOOLEAN  | PLAIN    | SNAPPY      | null | null       |
+--------------------------+-------+---------------+----------+----------+-------------+------+------------+
IOTDB#> 
IOTDB#> INSERT INTO root.ln.wf01.wt01(timestamp,status) values(100,true);
IOTDB#> INSERT INTO root.ln.wf01.wt01(timestamp,status,temperature) values(200,false,20.71)
IOTDB#> 
IOTDB#> SELECT status FROM root.ln.wf01.wt01
+--------------------------------+--------------------------+
| Time                           | root.ln.wf01.wt01.status |
+================================+==========================+
| 1970-01-01 08:00:00.100 +08:00 | true                     |
+--------------------------------+--------------------------+
| 1970-01-01 08:00:00.100 +08:00 | false                    |
+--------------------------------+--------------------------+
IOTDB#> SELECT * FROM root.ln.wf01.wt01
+--------------------------------+-------------------------------+--------------------------+
| Time                           | root.ln.wf01.wt01.temperature | root.ln.wf01.wt01.status |
+================================+===============================+==========================+
| 1970-01-01 08:00:00.100 +08:00 | null                          | true                     |
+--------------------------------+-------------------------------+--------------------------+
| 1970-01-01 08:00:00.100 +08:00 | 20.71                         | false                    |
+--------------------------------+-------------------------------+--------------------------+
```