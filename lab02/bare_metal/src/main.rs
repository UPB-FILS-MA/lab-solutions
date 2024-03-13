#![no_main]
#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

use cortex_m_rt::entry;

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    // reset IO Bank0
    const RESET: u32 = 0x4000_c000;
    const CLR: u32 = 0x3000;

    const RESET_DONE: u32 = 0x4000_c008;

    unsafe {
        write_volatile((RESET + CLR) as *mut u32, 1 << 5);
        while read_volatile(RESET_DONE as *const u32) & (1 << 5) == 0 {}
    }

    const GPIOX_CTRL: u32 = 0x4001_4004;
    const GPIO_OE_SET: *mut u32 = 0xd000_0024 as *mut u32;
    const GPIO_OUT_SET: *mut u32 = 0xd000_0014 as *mut u32;
    const GPIO_OUT_CLR: *mut u32 = 0xd000_0018 as *mut u32;

    const LED: u32 = 25;
    let mut value = 1;
    let gpio_ctrl = (GPIOX_CTRL + 8 * LED) as *mut u32;
    unsafe {
        write_volatile(gpio_ctrl, 5);
        write_volatile(GPIO_OE_SET, 1 << LED);
    };

    loop {
        value = 1 - value;
        let reg = match value {
            0 => GPIO_OUT_CLR,
            _ => GPIO_OUT_SET,
        };
        unsafe { write_volatile(reg, 1 << LED) }
        for _ in 0..50000 {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
