#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};
use cortex_m_rt::exception;

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_rt::entry;

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const SYST_RVR: *mut u32 = 0xe000_e014 as *mut u32;
const SYST_CVR: *mut u32 = 0xe000_e018 as *mut u32;
const SYST_CSR: *mut u32 = 0xe000_e010 as *mut u32;
// + 0x2000 is bitwise set
const SYST_CSR_SET: *mut u32 = (0xe000_e010 as u32 + 0x2000) as *mut u32;

// reset IO Bank0
const RESET: u32 = 0x4000_c000;
const CLR: u32 = 0x3000;

const RESET_DONE: u32 = 0x4000_c008;

const GPIOX_CTRL: u32 = 0x4001_4004;
const GPIO_OE_SET: *mut u32 = 0xd000_0024 as *mut u32;
const GPIO_OUT_SET: *mut u32 = 0xd000_0014 as *mut u32;
const GPIO_OUT_CLR: *mut u32 = 0xd000_0018 as *mut u32;

const LED: u32 = 0;

static ATOMIC_BOOL: AtomicBool = AtomicBool::new(true);

#[exception]
fn SysTick() {
    let atomic_bool = ATOMIC_BOOL.load(Ordering::Relaxed);
    let reg = match atomic_bool {
        false => GPIO_OUT_CLR,
        true => GPIO_OUT_SET,
    };

    unsafe { write_volatile(reg, 1 << LED) }

    ATOMIC_BOOL.store(!atomic_bool, Ordering::Relaxed);
}

#[entry]
fn main() -> ! {
    // enable the IO_BANK0 peripheral
    unsafe {
        write_volatile((RESET + CLR) as *mut u32, 1 << 5);
        while read_volatile(RESET_DONE as *const u32) & (1 << 5) == 0 {}
    }

    // set the LED pin the SIO function in IO_BANK0
    let gpio_ctrl = (GPIOX_CTRL + 8 * LED) as *mut u32;
    unsafe {
        write_volatile(gpio_ctrl, 5);
    };

    // set the LED pin as output in SIO
    unsafe {
        write_volatile(GPIO_OE_SET, 1 << LED);
    };

    // unsafe { write_volatile(GPIO_OUT_SET, 1 << LED) }

    // fire systick every 5 seconds
    let interval: u32 = 1_000_000;
    unsafe {
        write_volatile(SYST_RVR, interval);
        write_volatile(SYST_CVR, 0);
        // set fields `ENABLE`(<< 0) and `TICKINT`(<< 1)
        write_volatile(SYST_CSR, 0b11);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
