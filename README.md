# rgrep

A fast, multi-threaded grep clone written in Rust.

## Features

- **Pattern matching** - Search for text patterns in files
- **Multi-file search** - Search across multiple files simultaneously
- **Multi-threaded** - Each file is searched in parallel using OS threads
- **Line numbers** - Optional line number display with `-n` flag

## Installation

```bash
# Clone the repository
git clone <repo-url>
cd rgrep

# Build in release mode
cargo build --release

# Binary will be at ./target/release/rgrep
```

## Usage

```bash
rgrep [OPTIONS] <PATTERN> [FILES]...
```

### Arguments

| Argument | Description |
|----------|-------------|
| `PATTERN` | The text pattern to search for |
| `FILES` | One or more files to search in |

### Options

| Option | Description |
|--------|-------------|
| `-n, --line-numbers` | Display line numbers for each match |
| `-h, --help` | Print help information |

## Examples

```bash
# Search for "error" in a single file
rgrep error log.txt

# Search with line numbers
rgrep -n error log.txt

# Search across multiple files
rgrep error *.log

# Search for "TODO" in source files
rgrep -n TODO src/*.rs
```

### Sample Output

```bash
$ rgrep -n hello test1.txt test2.txt
test1.txt:1:hello world
test2.txt:3:hello rust
```

## How It Works

1. Parses command-line arguments using `clap`
2. Wraps the search pattern in `Arc` for thread-safe sharing
3. Spawns a separate thread for each input file
4. Each thread reads its file and prints matching lines
5. Main thread waits for all search threads to complete

## Dependencies

- [clap](https://crates.io/crates/clap) - Command-line argument parsing

## License

MIT
