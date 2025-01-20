#!/bin/sh

# Cargo Watch watches over your project's source for changes and runs Cargo commands when they occur.
# https://crates.io/crates/cargo-watch

if ! command -v cargo-watch &> /dev/null; then
    cargo install cargo-watch || exit 1
    sleep 1
fi

cargo-watch -q -c -w ./ -x "test -- --nocapture"
