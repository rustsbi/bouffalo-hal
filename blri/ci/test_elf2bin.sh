#!/usr/bin/env bash
set -euo pipefail

# List of all example packages
EXAMPLES=(
  gpio-demo
  i2c-demo
  jtag-demo
  lz4d-demo
  psram-demo
  pwm-demo
  sdcard-demo
  sdcard-gpt-demo
  sdh-demo
  sdh-dma-demo
  spi-demo
  uart-demo
  uart-async-demo
  uart-cli-demo
)

# Place comparison directories under target/ so they are ignored by git
COMPARISON_DIR="target/elf2bin-comparison"
RUST_OBJCOPY_BINDIR="$COMPARISON_DIR/rust-objcopy-bin"
BLRI_BINDIR="$COMPARISON_DIR/blri-bin"

mkdir -p "$RUST_OBJCOPY_BINDIR" "$BLRI_BINDIR"

echo "=== Building example targets ==="
for ex in "${EXAMPLES[@]}"; do
  # The build process should be completed before this script is run in CI,
  # since the `test-elf2bin-conversion` job the `ci.yml` file has some
  # build commands as `needs`.
  # Uncomment the following lines if you want to build the examples here
  # echo "Building $ex..."
  # cargo build --target riscv64imac-unknown-none-elf --release -p "$ex"

  # Path to the built ELF file
  ELF_PATH="target/riscv64imac-unknown-none-elf/release/riscv64-${ex}/${ex}"

  echo "Processing ${ex} from ${ELF_PATH}..."

  # Create bin with rust-objcopy
  rust-objcopy -O binary "$ELF_PATH" "$RUST_OBJCOPY_BINDIR/$ex.bin"

  # Create bin with our elf_to_bin functionality
  ./target/debug/blri elf2bin "$ELF_PATH" -o "$BLRI_BINDIR/$ex.bin"
done

echo "=== Comparing binary outputs ==="
FAILED=0

for ex in "${EXAMPLES[@]}"; do
  echo -n "Comparing $ex... "
  if cmp -s "$RUST_OBJCOPY_BINDIR/$ex.bin" "$BLRI_BINDIR/$ex.bin"; then
    echo "PASS"
  else
    echo "FAIL"
    FAILED=1
  fi
done

if [ $FAILED -eq 0 ]; then
  echo "=== All binary conversions match! ==="
  exit 0
else
  echo "=== Some binary conversions do not match! ==="
  exit 1
fi
