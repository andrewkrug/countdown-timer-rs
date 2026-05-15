#!/usr/bin/env bash
# A 10-second countdown — handy for verifying the build works end to end.
set -euo pipefail

cd "$(dirname "$0")/.."

cargo build --release
exec ./target/release/countdown-timer-rs 0.1667
