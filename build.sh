#!/usr/bin/env bash

set -euo pipefail


function step {
  echo " "
  echo " "
  echo "$1  =============================="
  echo " "
}

rustup update

step "LINTING"
rustup component add clippy
cargo clippy


step "TESTING"
cargo +stable install cargo-llvm-cov --locked
cargo clean
cargo llvm-cov # --text # --html


step "BUILDING DOCS "
cargo test --doc -- --show-output
RUSTDOCFLAGS="--html-in-header ./katex.html" cargo doc --verbose


step "BUILDING RELEASE"
cargo build --release


exit 0