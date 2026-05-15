#!/usr/bin/env bash
# A full Pomodoro cycle: four 25-minute work blocks separated by short
# breaks, with a longer break at the end. Press any key to advance past
# each "TIME'S UP!" screen.
set -euo pipefail

cd "$(dirname "$0")/.."

cargo build --release
TIMER=./target/release/countdown-timer-rs

WORK=25
SHORT_BREAK=5
LONG_BREAK=15

for round in 1 2 3 4; do
    echo "== Round $round: work for $WORK minutes =="
    "$TIMER" "$WORK"

    if [ "$round" -eq 4 ]; then
        echo "== Long break: $LONG_BREAK minutes =="
        "$TIMER" "$LONG_BREAK"
    else
        echo "== Short break: $SHORT_BREAK minutes =="
        "$TIMER" "$SHORT_BREAK"
    fi
done

echo "Pomodoro cycle complete."
