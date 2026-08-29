#!/usr/bin/env sh
# Keep the documented Rust minimum executable, rather than merely declared.
set -eu

rustup toolchain install 1.88.0 --profile minimal
cargo +1.88.0 test --locked
