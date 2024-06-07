pub fn cycles(num: u32) {
    for _ in 0..num {
        cortex_m::asm::nop();
    }
}