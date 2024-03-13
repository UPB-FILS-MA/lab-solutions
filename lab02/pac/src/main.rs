#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

use cortex_m_rt::entry;
use rp2040_pac::Peripherals;

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };

    let reset = peripherals.RESETS;
    reset
        .reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });
    while reset.reset_done().read().bits() & (1 << 5) == 0 {}

    const LED: usize = 25;

    let io_bank0 = peripherals.IO_BANK0;
    io_bank0
        .gpio(LED)
        .gpio_ctrl()
        .modify(|_, w| unsafe { w.funcsel().bits(5) });

    let sio = peripherals.SIO;
    sio.gpio_oe_set().write(|w| unsafe { w.bits(1 << LED) });

    let mut value = 1;
    loop {
        value = 1 - value;
        match value {
            0 => sio.gpio_out_clr().write(|w| unsafe { w.bits(1 << LED) }),
            _ => sio.gpio_out_set().write(|w| unsafe { w.bits(1 << LED) }),
        }

        for _ in 0..50000 {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}
