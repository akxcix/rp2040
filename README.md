# RP2040
Examples of programming RP2040 based boards using rust.

## Components Used
> NOTE: I have used the following components but i think this would be compatible with any RP2040 based board.

Needed:
- Adafruit Feather RP2040 RFM69: [link](https://www.adafruit.com/product/5712)

Optional:
- Adafruit MPU-6050: [link](https://www.adafruit.com/product/3886)

## Getting Started
### Setting Up Rust
```
rustup target add thumbv6m-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview

cargo install elf2uf2-rs
cargo build --release
```
