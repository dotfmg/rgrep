# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `rgrep`, a minimal grep implementation in Rust that searches text in files or STDIN. The project follows a simple CLI pattern with argument parsing, configuration management, and text search functionality.

## Architecture

- **Entry Point**: `src/main.rs` - Handles command-line argument parsing and delegates to library functions
- **Core Library**: `src/lib.rs` - Contains the main logic for:
  - `Config` struct for storing search parameters (pattern, files, flags)
  - `parse_args()` for command-line argument parsing
  - `run()` for executing the search operation
  - `search_reader()` for performing text matching on input streams
- **Tests**: Located in `test/` directory with separate modules for parsing and search functionality

## Key Components

- **Action Enum**: Represents different CLI actions (ShowHelp, ShowVersion, Run)
- **Config Struct**: Stores search configuration (pattern, files, ignore_case, line_number)
- **ParseError**: Custom error type for argument parsing failures
- **Text Search**: Case-sensitive/insensitive substring matching with optional line numbers

## Development Commands

```bash
# Build the project
cargo build

# Run tests
cargo test

# Check for compilation errors without building
cargo check

# Run the CLI tool
cargo run -- <pattern> [files...]

# Run with flags
cargo run -- -i -n <pattern> [files...]
```

## Testing

Tests are organized in the `test/` directory:
- `test/parse.rs` - Tests for argument parsing functionality
- `test/search.rs` - Tests for search functionality and exit codes

Run specific test modules:
```bash
cargo test parse
cargo test search
```

## Current Features

- Substring search in files or STDIN
- Case-insensitive matching (`-i`, `--ignore-case`)
- Line number display (`-n`, `--line-number`)
- Multiple file support with filename prefixes
- Proper exit codes (0 for matches, 1 for no matches, 2 for errors)