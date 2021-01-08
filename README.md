# actix-cli
CLI to create web services with Actix

- [x] Minimal server with health check and readiness check
- [x] Fault-tolerant option can be enabled to use Bastion
- [x] Logger middleware option can be enabled with `--request-logger` flag
- [ ] Read routes configs from `Config.toml`
- [ ] Basic CRUD controllers
- [ ] Auth middleware option can be enabled with `--auth` flag
- [ ] Read models configs from `Config.toml`
- [ ] Basic Docker config can be enabled with flag

> Not defining a Database will mean a `Context` will be created to support a basic `HashMap`.
 

## Installing

1. If you don't have Rust in your computer you can run `make setup` or visit https://rustup.rs/.
2. Make sure your `/usr/local/bin` is in `PATH`.
3. Than run `make build`.

> * Windows not yet supported

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