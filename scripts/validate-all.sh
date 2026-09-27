#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo clippy --all-targets
cargo test
cargo run -p validate-fixtures
node --test scripts/bundle-schema.test.mjs
scripts/generate-typescript.sh
scripts/check-wire-casing.sh
