#!/usr/bin/env bash

set -euo pipefail

echo " "
echo " "
echo "TESTING + COVERAGE  ================="
echo " "
cargo install cargo-llvm-cov
cargo clean
cargo llvm-cov # --text # --html


echo " "
echo " "
echo "BUILDING DOCS  ======================"
echo " "
RUSTDOCFLAGS="--html-in-header ./katex.html" cargo doc --verbose


echo " "
echo " "
echo "BUILDING RELEASE  ==================="
echo " "
cargo build --release


exit 0