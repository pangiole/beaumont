#!/usr/bin/env bash

set -euo pipefail
readonly script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"


function step {
  echo " "
  echo " "
  echo "$1  =============================="
  echo " "
}

rustup update

step "CLEANING"
cargo install cargo-cache
cargo cache --autoclean
cargo clean


step "LINTING"
rustup component add clippy
cargo clippy


step "TESTING"
cargo +stable install cargo-llvm-cov --locked
cargo +stable install cargo-expand --locked
cargo clean
cargo llvm-cov # --text # --html


step "BUILDING DOCS "
cargo test --doc -- --show-output
cargo doc --verbose --no-deps


step "BUILDING RELEASE"
cargo build --release


exit 0