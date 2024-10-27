# Tago

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.1-blue)](https://crates.io/crates/tago)

# Example

```
❯ tago all
feature-tago-delete: 12 hours, 38 minutes, 13 seconds, 915 ms and 963 µs
default: 2 months, 1 week, 3 days, 21 hours, 33 minutes, 4 seconds, 660 ms and 914 µs
cairnify/login: 34 seconds, 527 ms and 736 µs
```

## Installation

### Cargo

```
cargo install tago
```

## Usage

Stores the current time under name.


```
tago init <name>
```

Shows the time elapsed since the saved time for name.


```
tago days <name> 
```

> [!TIP]
> If the name is omitted, it will default to `default`.
>

Displays all saved time entries.


```
tago all
```

Removes all saved time entries.


```
tago clean
```

## Misc

All timestamps are saved in `~/.config/tago/config.toml`.