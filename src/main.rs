#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::nop();

    loop {
        // your code goes here
    }
}
