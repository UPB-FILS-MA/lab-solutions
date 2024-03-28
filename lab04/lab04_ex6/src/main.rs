#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};
use cortex_m_rt::exception;

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_rt::entry;

// TODO 1: Create RP2040 boot loader

// TODO 2: Initialize registers for LED, SIO function and the IO bank

// TODO 5: Initialize registers for SysTick

// TODO 3: Define LED number

// TODO 7: Define static atomic bool

#[exception]
fn SysTick() {
    // TODO 8: Match current atomic bool value and turn LED on or off

    // TODO 9: Change bool value
}

#[entry]
fn main() -> ! {
    // TODO 4: Configure LED

    // TODO 6: Enable systick every 100 ms (set SYST_RVR, SYST_CVR, SYST_CSR fields ENABLE and TICKINT)

    loop {
        // delete this otherwise it will panic
        todo!();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
