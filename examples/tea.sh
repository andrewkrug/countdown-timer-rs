#!/usr/bin/env bash
# A 3-minute tea steeping timer.
set -euo pipefail

cd "$(dirname "$0")/.."

cargo build --release
exec ./target/release/countdown-timer-rs 3
