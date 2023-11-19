_default:
  @just --list

# Project setup
setup:
  # Istall pre compiled rust crates
  cargo install cargo-binstall

  # Install nextest as cargo test replacement
  cargo binstall cargo-nextest

# Tests
test:
  # nextest runs unit and compiles benchmark tests
  cargo nextest run --workspace --all-targets

# Code format
format:
  cargo +nightly fmt --all

# Clippy
check:
  cargo clippy --workspace --all-targets -- -D warnings -W clippy::all
