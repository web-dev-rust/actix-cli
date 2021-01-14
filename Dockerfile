FROM rust:latest

COPY Cargo.* ./
COPY src/ ./src

RUN cargo build --release
RUN cat target/release/actix-cli > actix-cli
ENTRYPOINT [ "/bin/bash" ]