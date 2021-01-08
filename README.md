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
```

* `fault-tolerant`: `bool`.
* `name`: `String`.