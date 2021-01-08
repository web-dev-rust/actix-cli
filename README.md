# actix-cli
CLI to create web services with Actix

## Usage

```sh
actix-cli 0.1.0
A basic example

USAGE:
    actix-cli [OPTIONS] --name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fault-tolerant <fault-tolerant>    Enables Bastion for fault tolerant system [default: true]
    -n, --name <name>                        Defines project name in Cargo.toml
    -r, --request-logger <request-logger>    Enables request logger as `[IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s
                                             DURATION:%D X-REQUEST-ID:%{x-request-id}o] and `"[x-request-id:
                                             Uuid::new_v4()]` [default: true]
```

* `fault-tolerant`: `bool`.
* `request-logger`: `bool`.
* `name`: `String`.