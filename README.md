# countdown-timer-rs

A colorful, full-screen ASCII countdown timer for your terminal. Big block
digits, a progress bar, color stages, and a flashing alarm when time's up.

```
 ██████   ██████   ██    ██    ██████
██    ██ ██    ██  ██    ██   ██    ██
██    ██ ██    ██  ██    ██   ██    ██
██    ██  ██████   ████████    ██████
██    ██ ██    ██        ██   ██    ██
██    ██ ██    ██        ██   ██    ██
 ██████   ██████         ██    ██████
▐████████████████████████░░░░░░░░░░░░▌
            ⏳  4 minutes remaining
```

## Features

- Large ASCII block-digit clock, centered in the terminal
- Auto-switches between `MM:SS` and `HH:MM:SS` for long countdowns
- Color stages as time runs down: green → cyan → yellow → red
- Bold flashing alarm during the final 2 minutes
- Live progress bar
- Flashing `⏰ TIME'S UP! ⏰` finish, dismissed with any key
- Quit early with `q` or `Esc`
- Accepts fractional minutes (e.g. `0.5` for 30 seconds)

## Install

Requires a [Rust toolchain](https://rustup.rs/).

```bash
git clone https://github.com/andrewkrug/countdown-timer-rs.git
cd countdown-timer-rs
cargo build --release
# binary at ./target/release/countdown-timer-rs
```

Or install it onto your `PATH`:

```bash
cargo install --path .
```

## Usage

```bash
countdown-timer-rs <MINUTES>
```

`<MINUTES>` is a positive number; decimals are allowed.

| Command                          | Countdown          |
| -------------------------------- | ------------------ |
| `countdown-timer-rs 5`           | 5 minutes          |
| `countdown-timer-rs 0.5`         | 30 seconds         |
| `countdown-timer-rs 25`          | Pomodoro work block|
| `countdown-timer-rs 90`          | 1 hour 30 minutes  |

While running:

- `q` / `Q` / `Esc` — quit early
- any key — dismiss the alarm once it finishes

Via the Makefile:

```bash
make release                       # optimized build
make run-args ARGS="3"             # run a 3-minute timer
```

## Examples

Runnable shell scripts live in [`examples/`](examples/):

- [`pomodoro.sh`](examples/pomodoro.sh) — a full Pomodoro cycle (4 × 25 min work + breaks)
- [`tea.sh`](examples/tea.sh) — a 3-minute steeping timer
- [`quick.sh`](examples/quick.sh) — a 10-second smoke test of the build

```bash
./examples/tea.sh
```

## License

Licensed under the [Apache License 2.0](LICENSE).
