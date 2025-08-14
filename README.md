# loc

A simple CLI tool for counting lines of code, written in Rust.

## Usage

Run the binary with optional flags to scan a directory and count lines of code.

Synopsis:

```
loc [OPTIONS]
```

Options:

- -p, --path <PATH>
  Directory to scan for source files (defaults to current directory)

- -n, --no-ignore  [alias: disable-ignore]
  Disable default ignore patterns

Examples:

- Scan the current directory:
  `loc`

- Scan a specific directory:
  `loc -p ./src`

- Disable default ignore patterns:
  `loc --no-ignore`

- Windows path example:
  `loc --path C:\\Projects\\my-app`

From source:

- Build: `cargo build --release`
- Run: `cargo run -- [OPTIONS]`

## Features

- Counts lines of code in supported languages
- Ignores common build and dependency directories by default
- Option to disable ignore patterns