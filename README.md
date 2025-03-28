# Epoch

A minimal terminal clock that displays the current time in a clean, distraction-free format.

## Features

- Simple, clean display of the current time
- Minimal resource usage
- Clean terminal exit (preserves command history)
- Raw terminal mode for immediate updates

## Installation

### From Source
```bash
# Clone the repository
git clone https://github.com/roanm/epoch.git
cd epoch

# Build with Cargo
cargo build --release

# The binary will be available at target/release/epoch
```

## Usage
```
A simple clock screen saver.

Usage: epoch [OPTIONS]

Options:
  -y, --year         Display the year.
  -m, --month        Display the month.
  -d, --day          Display the day.
  -s, --second       Display the seconds.
  -f, --millisecond  Display the milliseconds.
  -h, --help         Print help
  -V, --version      Print version

```
