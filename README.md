## os requirement

Indications for debian based

    sudo apt-get install libudev-dev libssl-dev
    rustup update
    cargo install cargo-edit
    cargo add async-std

## build

    cargo build
    
## run

    cargo run

## logging

See https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html

for dev, use `RUST_LOG="rollac=debug,rollac::event=info"`
