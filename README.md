# rgrep

A fast, multi-threaded grep clone written in Rust.

## Features

- **Regex support** - Full regular expression pattern matching
- **Multi-file search** - Search across multiple files simultaneously
- **Multi-threaded** - Each file is searched in parallel using OS threads
- **Recursive search** - Search directories recursively with `-r`
- **Case insensitive** - Optional case insensitive search with `-i`
- **Line numbers** - Optional line number display with `-n`
- **Colored output** - Highlights matches in red/bold
- **Respects .gitignore** - Automatically skips ignored files and directories

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
| `PATTERN` | Regex pattern to search for |
| `FILES` | One or more files or directories to search in |

### Options

| Option | Description |
|--------|-------------|
| `-n, --line-numbers` | Display line numbers for each match |
| `-i, --case-insensitive` | Case insensitive search |
| `-r, --recursive` | Recursively search directories |
| `-h, --help` | Print help information |

## Examples

```bash
# Basic search
rgrep error log.txt

# Search with line numbers
rgrep -n error log.txt

# Case insensitive search
rgrep -i ERROR log.txt

# Search across multiple files
rgrep error *.log

# Recursive search in a directory
rgrep -r "fn main" src/

# Combine flags
rgrep -rni "todo" .

# Regex patterns
rgrep "err(or|no)" log.txt        # matches "error" or "errno"
rgrep "^import" src/*.py          # lines starting with "import"
rgrep "[0-9]{3}-[0-9]{4}" data/   # phone number pattern
```

### Sample Output

```
$ rgrep -rn "fn main" src/
src/main.rs:44:fn main() {
```

Matches are highlighted in red/bold in terminal output.

## How It Works

1. Parses command-line arguments using `clap`
2. Compiles the regex pattern and wraps it in `Arc` for thread-safe sharing
3. If recursive, walks directories respecting `.gitignore` patterns
4. Spawns a separate thread for each input file
5. Each thread reads its file, matches lines, and prints with colored highlights
6. Main thread waits for all search threads to complete

## Dependencies

- [clap](https://crates.io/crates/clap) - Command-line argument parsing
- [regex](https://crates.io/crates/regex) - Regular expression engine
- [colored](https://crates.io/crates/colored) - Terminal colors
- [walkdir](https://crates.io/crates/walkdir) - Recursive directory traversal

## License

MIT
