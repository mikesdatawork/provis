# Provis

A snapshot-based system monitoring tool for Linux showing filesystem usage and process resource consumption.

**Note:** Provis displays a single snapshot of current system state. It is not a continuously updating real-time monitor. Run the command each time you want an updated view.

Forked from [dysk](https://github.com/Canop/dysk) and enhanced with process monitoring capabilities.

## Features

### Disk View (Default)
Display filesystem information with usage statistics.

```bash
provis
```

Shows:
- Filesystem type and mount points
- Disk usage with visual bars
- Available space
- Total size
- Color-coded usage indicators (red for used space)

### Process View
Display top processes by CPU usage.

```bash
provis --processes
# or
provis -p
```

Shows:
- Top 10 processes by CPU usage
- Process ID (PID) and name
- CPU percentage with visual bars
- Memory percentage with visual bars
- Disk read/write activity
- Thread aggregation (all threads combined per process)
- Self-filtering (provis itself excluded from results)

## Command-Line Options

### View Selection
- `--processes`, `-p` - Show process view instead of disk view

### Display Options
- `--color <auto|yes|no>` - Control color output (default: auto)
- `--ascii` - Use ASCII characters instead of Unicode for bars
- `--all`, `-a` - Show all filesystems (disk view only)
- `--cols`, `-c` - Customize columns in disk view (e.g., `-c label+default+dev`)

### Filtering and Sorting (Disk View)
- `--filter`, `-f` - Filter filesystems (e.g., `-f 'size>100G'`)
- `--sort`, `-s` - Sort by column (e.g., `-s free`)
- `--units` - Display units: SI, binary, or bytes

### Output Formats (Disk View)
- `--json`, `-j` - Output as JSON
- `--csv` - Output as CSV
- `--csv-separator` - Set CSV separator (default: comma)

### Information
- `--help` - Show help information
- `--version` - Show version
- `--list-cols` - List available columns for disk view

## Examples

### Disk Usage
```bash
# Standard disk view
provis

# All filesystems including system mounts
provis --all

# Filter for low space
provis -f 'free < 50G'

# Custom columns
provis -c label+fs+use+size+mount

# Export as JSON
provis --json > disks.json
```

### Process Monitoring
```bash
# Show top CPU-intensive processes
provis --processes

# Without color
provis -p --color no

# ASCII mode for compatibility
provis -p --ascii
```

## Installation

### From Source
```bash
# Clone repository
git clone https://github.com/mikesdatawork/provis.git
cd provis

# Build release version
cargo build --release

# Binary location
./target/release/provis
```

### System-wide Installation
```bash
# Copy to system path
sudo cp target/release/provis /usr/local/bin/

# Or create alias
alias provis='/path/to/provis/target/release/provis'
```

## Technical Details

### Process Monitoring
- Uses sysinfo crate for cross-platform process information
- Reads `/proc` filesystem on Linux for I/O statistics
- CPU measurements require 200ms delta for accuracy
- Thread aggregation combines all threads per process
- Disk I/O tracked via `/proc/[pid]/io` (Linux only)

### Color Scheme
- Red (ANSI 209) for usage bars and percentages
- Default terminal color for other text
- Automatically disabled when piping output
- Compatible with both light and dark terminal themes

### Performance
- Release builds recommended for production use
- Debug builds are 10-50x slower
- Minimal overhead (typically <5% CPU when running)
- Single snapshot - no continuous monitoring overhead

## Roadmap

Planned features:
- Directory size analysis (TreeSize-like functionality)
- Configurable refresh rates for watch mode
- Process filtering and sorting options
- Historical trending with sparklines
- Network usage per process
- Process tree visualization

## Requirements

- Linux (primary target)
- Rust 1.75+ for building
- Terminal with Unicode support (or use `--ascii` flag)

## License

MIT License - see LICENSE file.

## Credits

Built on top of [dysk](https://github.com/Canop/dysk) by Denys Séguret.

Process monitoring enhancements by [mikesdatawork](https://github.com/mikesdatawork).
