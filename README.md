# System Index

A comprehensive CLI and TUI tool for displaying system information, written in Rust.

## Features

- 🖥️ Interactive Terminal User Interface (TUI) with multiple views
- 🔧 Command Line Interface (CLI) for quick information retrieval
- 💾 Memory and swap usage monitoring
- 💿 Disk space and filesystem information
- 🌐 Network interface statistics
- ⚙️ CPU and system details
- 🧪 Comprehensive test suite
- 🚀 CI/CD with GitHub Actions
- 📦 Cross-platform releases (Linux, macOS, Windows)
- 🔒 Security auditing

## Installation

### From Source

```bash
git clone https://github.com/npsg02/system-index.git
cd system-index
cargo build --release
```

### From Releases

Download the latest binary from the [Releases](https://github.com/npsg02/system-index/releases) page.

## Usage

### Command Line Interface

```bash
# Show help
./system-index --help

# Display system overview (default)
./system-index overview

# Display CPU information
./system-index cpu

# Display memory information
./system-index memory

# Display disk information
./system-index disks

# Display network information
./system-index network

# Display all system information
./system-index all

# Start interactive TUI
./system-index tui
```

### Terminal User Interface (TUI)

Start the interactive mode:

```bash
./system-index tui
# or simply
./system-index
```

#### TUI Commands:
- `h` - Show help
- `r` - Refresh system information
- `1` - Show system overview
- `2` - Show memory details
- `3` - Show disk information
- `4` - Show network information
- `q` - Quit application

#### TUI Features:
- **Overview Tab**: Displays hostname, OS, kernel, uptime, CPU, memory, and summary statistics
- **Memory Tab**: Shows detailed RAM and swap usage with visual bars
- **Disks Tab**: Lists all mounted disks with capacity and usage information
- **Network Tab**: Displays network interfaces with data transfer statistics
- **Auto-refresh**: System information automatically updates every 2 seconds

## Project Structure

```
system-index/
├── .github/workflows/    # CI/CD workflows
├── src/
│   ├── models/           # Data models (SystemInfo)
│   ├── tui/              # Terminal UI implementation
│   ├── lib.rs            # Library root
│   └── main.rs           # CLI application
├── tests/                # Integration tests
├── docs/                 # Documentation
└── examples/             # Usage examples
```

## Development

### Prerequisites

- Rust 1.70 or later

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Clippy (Linter)

```bash
cargo clippy -- -D warnings
```

### Formatting Code

```bash
cargo fmt
```

## System Information Collected

The tool collects and displays:

- **Operating System**: Name, version, kernel version, hostname
- **CPU**: Brand, number of cores
- **Memory**: Total RAM, used RAM, free RAM, swap usage
- **Disks**: All mounted filesystems with capacity and usage
- **Network**: All network interfaces with received/transmitted data
- **Processes**: Count of running processes
- **Uptime**: System uptime in human-readable format

## CI/CD

The project includes comprehensive GitHub Actions workflows:

- **CI**: Build, test, lint, and format checks on multiple platforms
- **Security**: Weekly security audits with `cargo audit`
- **Release**: Automated binary releases for Linux, macOS, and Windows

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
