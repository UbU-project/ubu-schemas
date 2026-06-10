#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo clippy --all-targets
cargo test
cargo run -p validate-fixtures
scripts/generate-typescript.sh

