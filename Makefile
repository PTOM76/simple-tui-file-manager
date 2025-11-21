all: build deb

build:
	cargo build --release

deb:
	cargo deb

exec:
    cargo run

install:
	sudo dpkg -i target/debian/*.deb
