#!/usr/bin/env bash

set -euo pipefail

cargo clean
cargo test
cargo doc
cargo build --release

exit 0