# actix-cli
CLI to create web services with Actix

- [x] Minimal server with health check and readiness check
- [x] Fault-tolerant option can be enabled to use Bastion
- [x] Logger middleware option can be enabled with `--request-logger` flag
- [x] Basic CRUD controllers
- [ ] `Unwrap`s to Crate Error
- [ ] Database configurations
- [ ] Auth middleware option can be enabled with `--auth` flag
- [ ] Basic Docker config can be enabled with flag
- [ ] Read routes configs from `Config.toml`
- [ ] Read models configs from `Config.toml`


> Not defining a Database will mean a `Context` will be created to support a basic `HashMap`.
 

## Installing

1. If you don't have Rust in your computer you can run `make setup` or visit https://rustup.rs/.
2. Make sure your `/usr/local/bin` is in `PATH`.
3. Than run `make build`.

> * Windows not yet supported

## Usage

```sh
actix-cli 0.1.0
A CLI to create actix-web projects boilerplate

USAGE:
    actix-cli [OPTIONS] --context <context> --name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config-file <config-file>          Config.toml file path
        --context <context>                  Which database configuration. Currently only `InMemory` allowed [possible
                                             values: InMemory]
    -f, --fault-tolerant <fault-tolerant>    Enables Bastion for fault tolerant system [default: true]
    -n, --name <name>                        Defines project name in Cargo.toml
    -r, --request-logger <request-logger>    Enables request logger as `[IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s
                                             DURATION:%D X-REQUEST-ID:%{x-request-id}o] and `"[x-request-id:
                                             Uuid::new_v4()]` [default: true]
```

* `fault-tolerant`: `bool`.
* `request-logger`: `bool`.
* `name`: `String`.
* `config-file`: `std::path::PathBuf`.
* `context`: case insetive enum containing `InMemory` and in the future `PostgresDB` and `DynamoDB`.

### Example

By executing command `actix-cli -n maluco-cli -r true -f true --config-file actix-cli/Config.toml --context inmemory` with the Config.toml as defined below in **CRUD** section the result is found in the [repo](https://github.com/web-dev-rust/actix-cli-basic-example)

## Confit.toml

A few examples of `Config.toml`.

### CRUD

```toml
[crud]
name = "object"

[crud.routes]
create = "object/new"
read = "object/get/{id}"
update = "object/update/{id}"
delete = "object/delete/{id}"
list = "object/get"

[crud.model]
name = "String"
age = "usize"
school = "String"
```

* `name` is required.
* `routes` is required, all routes should be Strings.
* If `routes` don't start with `name`, `name` will bem added to them.
* `model` is required, all types should be Strings with valid Rust types.