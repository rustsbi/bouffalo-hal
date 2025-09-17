# blri

A command-line tool for Bouffalo Lab IoT chips that handles ROM image completion, patching, and flashing with integrated serial console support.

## Features

- **ELF to Binary Conversion**: Convert ELF files to binary format suitable for flashing
- **Image Patching**: Automatically fix CRC32 checksums and apply necessary corrections
- **Device Flashing**: Flash binary images to Bouffalo Lab chips via ISP (In-System Programming)
- **Serial Console**: Built-in serial console/monitor for debugging after flashing
- **Configuration Management**: Save and reuse flashing configurations for different projects
- **Smart Port Detection**: Automatically detect available serial ports

## Usage

### Basic Commands

```bash
# Convert ELF to binary and patch
blri elf2bin input.elf -o output.bin --patch

# Flash image to device
blri flash image.bin --port /dev/ttyUSB0 --reset

# One-step: convert, patch, flash, and open uart in console mode
blri run target/riscv64imac-unknown-none-elf/release/my-app --reset --console

# Use saved configuration for quick development
blri default
```

### Configuration Management

blri automatically saves successful configurations including:
- Target architecture
- Build mode (debug/release)
- Package name
- Serial port and baudrate
- Reset and console preferences

When conflicts are detected, blri will prompt:
1. **First confirmation**: "Use current configuration instead of saved configuration?"
2. **Second confirmation**: "Save current configuration for future use?"

### Serial Console

After flashing, you can open a serial console to interact with your device:

```bash
# Open console mode (interactive)
blri run my-app --console

# Open monitor mode (read-only)
blri run my-app
```

Console features:
- Real-time device output display
- Interactive command input (console mode)
- Configurable baudrate (default: 2000000 bps)
- Exit with Ctrl+C

### Integration with Cargo

Add to your `.cargo/config.toml`:

```toml
[alias]
blri = "run --package blri --release --"
blri-default = "run --package blri --release -- default"
```

Then use:

```bash
# From project root
cargo blri run target/riscv64imac-unknown-none-elf/release/my-app --reset --console

# Quick run with saved config
cargo blri-default
```

## Examples

### Development Workflow

```bash
# First time setup
cargo build --target riscv64imac-unknown-none-elf --release -p my-project
cargo blri run target/riscv64imac-unknown-none-elf/release/my-project --reset --console

# Subsequent runs
cargo build --target riscv64imac-unknown-none-elf --release -p my-project
cargo blri-default
```

### Flashing Different Projects

```bash
# Switch to a different project
cargo blri run target/riscv64imac-unknown-none-elf/release/uart-demo --reset

# blri will detect configuration differences and prompt for confirmation
```

## Configuration File

Configurations are automatically saved to `target/settings.toml`:

```toml
target = "riscv64imac-unknown-none-elf"
release = true
package = "my-project"
binary_path = "target/riscv64imac-unknown-none-elf/release/my-project"
port = "/dev/ttyUSB0"
baudrate = 2000000
reset = true
console = true
```

## License

This project is licensed under the MIT License - see the LICENSE files for details.
