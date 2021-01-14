setup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

build:
	cargo build --release
	cp target/release/actix-cli /usr/local/bin/actix-cli

build-linux:
	docker build --file Dockerfile --output out . || cp out/actix-cli ./