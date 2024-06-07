# rp2040
getting started with rp2040 programming from scratch
### sections
- [x] blink
- [ ] i2c display
- [ ] reading input

## details
i have tried to keep the setup as minimal as possible, with the disclaimer 
that i know very little about programming embedded systems. 

i have tried to have as few dependencies as possible and try to explain the 
neccessity for all of the ones that i actually end up using. 

i also try to explain the file structure and the responsibility of each 
particular part. i am not confident that this is the best way to abstract 
out and organise the code but this worked for my example, so there it is.

## dependencies
- [rp2040-hal](https://crates.io/crates/rp2040-hal): the main drivers for ro2040. implement the hardware abstraction layer methods as defines by `embedded-hal`. this is the lowest board specific dependency i wanted to have, but their are actually separate board support packages as well. this provides an easier way to manage the io, otherwise we would need to directly interact with the registers
- [rp2040-boot2](https://crates.io/crates/rp2040-boot2): the second stage bootloader. we need to prepare the board to use the application code, this bootloader does that. refer to this [guide](https://vanhunteradams.com/Pico/Bootloader/Boot_sequence.html) for more on this.
- [cortex-m](https://crates.io/crates/cortex-m): utilities and functions needed for accessing cortex-m based microcontrollers, which rp2040 is
- [cortex-m-rt](https://crates.io/crates/cortex-m-rt): minimal runtime for cortex-m microcontrollers, sets up startup code and vector table, ensuring the microcontroller has necessary low-level setup to start executing the application code
- [embedded-hal](https://crates.io/crates/embedded-hal): hardware abstraction layer. it defines a set of traits for abstracting the hardware which specific crates implement for specific boards (`rp2040-hal` in our case)
- [panic-halt](https://crates.io/crates/panic-halt): handling panics are hard in resource constrained environments such as embedded systems. this crate provides a handler that halts in case of any panic

## file structure
- [`.cargo/config.toml`](.cargo/config.toml): configuration for the compiler
- [`memory.x`](/memory.x): linker script used to define the memory layout of the microcontroller. tells the linker where to specific parts of the program in the memory, and allows us to exactly control how memory is used
- [`src/main.rs`](/src/main.rs): main entrypoint
- [`src/rp`](/src/rp/): the lib
- [`src/rp/delay.rs`](/src/rp/delay.rs): the operations for specifying delays. i use `nop`s for this right now, but there are possibilities to use the internal clock for this.
- [`src/rp/entry.rs`](/src/rp/entry.rs): the main code that runs on the rp2040. might be possible to abstract more of it away but i have not yet done that.

## components
i used the following components, but i am pretty sure that this would actually work with any other specific ones as well, if the pin assignments are correctly made. i think i might actually be able to abstract out the configs for different boards and use them while compilation, but that is besides the point. maybe in the future when i get time.

- adafruit feather rp2040 rfm69: [link](https://www.adafruit.com/product/5712)
- adafruit mpu-6050: [link](https://www.adafruit.com/product/3886)

## setting up the environment
### rust
```
rustup target add thumbv6m-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview

cargo install elf2uf2-rs
cargo build --release
```

### arm toolchain
i used the [guide](https://gist.github.com/disposedtrolley/06d37e1db82b80ccf8c5d801eaa29373) by [James Liu](https://github.com/disposedtrolley) for this.

the main code is compiled into an elf. we can use the arm toolchain to dump and examine the code as it is laid out. for example

```
/Applications/ARM/bin/arm-none-eabi-objdump -D target/thumbv6m-none-eabi/debug/rp2040 | grep .boot2 -A 25
```

will output

```
Disassembly of section .boot2:

10000000 <_ZN6rp20405BOOT217h2323e6183c50c2f1E>:
10000000:	4b0cb500 	blmi	1032d408 <__veneer_base+0x329a68>
10000004:	60992100 	addsvs	r2, r9, r0, lsl #2
10000008:	61592102 	cmpvs	r9, r2, lsl #2
1000000c:	6019490a 	andsvs	r4, r9, sl, lsl #18
10000010:	480b490a 	stmdami	fp, {r1, r3, r8, fp, lr}
10000014:	21006001 	tstcs	r0, r1
10000018:	21016059 	qaddcs	r6, r9, r1
1000001c:	bc016099 	stclt	0, cr6, [r1], {153}	; 0x99
10000020:	d0002800 	andle	r2, r0, r0, lsl #16
10000024:	48074700 	stmdami	r7, {r8, r9, sl, lr}
10000028:	60084907 	andvs	r4, r8, r7, lsl #18
1000002c:	f380c803 	vmlal.u8	q6, d0, d3
10000030:	47088808 	strmi	r8, [r8, -r8, lsl #16]
10000034:	18000000 	stmdane	r0, {}	; <UNPREDICTABLE>
10000038:	001f0300 	andseq	r0, pc, r0, lsl #6
1000003c:	03000218 	movweq	r0, #536	; 0x218
10000040:	180000f4 	stmdane	r0, {r2, r4, r5, r6, r7}
10000044:	10000100 	andne	r0, r0, r0, lsl #2
10000048:	e000ed08 	and	lr, r0, r8, lsl #26
	...
100000fc:	5d4c22ea 	sfmpl	f2, 2, [ip, #-936]	; 0xfffffc58

Disassembly of section .text:
```

you can see the contents of the second stage bootloader with this

## References:
- https://vanhunteradams.com/Pico/Bootloader/Boot_sequence.html
- https://gist.github.com/disposedtrolley/06d37e1db82b80ccf8c5d801eaa29373
- https://www.youtube.com/watch?v=jZT8APrzvc4
