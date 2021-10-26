<div align="center">

![Logo](https://raw.githubusercontent.com/francis-du/iotdb-rs/main/iotdb-rs.png)

<h1>iotdb-cli</h1>
<h3>(WIP) Apache IotDB CLI Client written in Rust</h3>

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
    ▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀      0.1.0
    
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

```shell
iotdb -u root -p root --endpoint 127.0.0.1:6667

▀██▀  ▄▄█▀▀██   █▀▀██▀▀█ ▀██▀▀█▄   ▀██▀▀█▄
 ██  ▄█▀    ██     ██     ██   ██   ██   ██
 ██  ██      ██    ██     ██    ██  ██▀▀▀█▄
 ██  ▀█▄     ██    ██     ██    ██  ██    ██
▄██▄  ▀▀█▄▄▄█▀    ▄██▄   ▄██▄▄▄█▀  ▄██▄▄▄█▀     

Connect server: 127.0.0.1:6667
Version: 0.1.0
IOTDB#> SHOW STORAGE GROUP
+---------------+
| storage group |
+===============+
| root.ln       |
+---------------+
| root.sg1      |
+---------------+
IOTDB#> 
```