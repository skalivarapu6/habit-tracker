# Habit Tracker

An experimental terminal habit tracker written in Rust. It supports streak
habits, quantity-based habits, JSON persistence, an interactive CLI, and an
early Ratatui interface.

## Run

```sh
cargo run
```

Start the terminal UI with:

```sh
cargo run -- --tui
```

Inside the CLI, enter `help` to list commands. Habit data is stored locally in
`habits.json`; that file is intentionally excluded from version control.

## Status

This is an early learning project. The CLI is the most complete interface, and
the Ratatui interface is still under development.
