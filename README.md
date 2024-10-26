# Tago

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