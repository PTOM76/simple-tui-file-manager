# simple-tui-file-manager

Simple file manager to practice using tui-rs

## Keymap

| Key      | description     |
| -------  | --------------- |
| j, up    | move up         |
| k, down  | move down       |
| h, Left  | move parent dir |
| l, Right | move child dir  |
| tab      | next dir tab    |
| S+tab    | prev dir tab    |
| q        | quick puit      |

TODO: Update as needed

## Installation

cargo build only

```sh
git clone https://github.com/GreasySlug/simple-tui-file-manager.git
rustup update
cd simle-tui-file_manager
cargo build
```

## To Contribute

This is a project for practicing github and rust.

Please check this [doc](/docs/contribute.md)

## Setup
```sh
sudo apt update
sudo apt install build-essential gcc g++ make gcc-aarch64-linux-gnu libc6-dev-arm64-cross
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
```

## Makefile
### Build

```sh
make build
make deb
```

or 

```sh
make all
```

### Install

```sh
make install
```

### Execute

```sh
make exec
```
