#![no_main]
#![no_std]

use core::arch::asm;
use core::convert::Infallible;
use core::panic::PanicInfo;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

use cortex_m_rt::entry;
use embedded_hal::digital::ErrorType;
use embedded_hal::digital::OutputPin;

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// reset IO Bank0
const RESET: u32 = 0x4000_c000;
const CLR: u32 = 0x3000;

const RESET_DONE: u32 = 0x4000_c008;

const GPIOX_CTRL: u32 = 0x4001_4004;
const GPIO_OE_SET: *mut u32 = 0xd000_0024 as *mut u32;
const GPIO_OUT_SET: *mut u32 = 0xd000_0014 as *mut u32;
const GPIO_OUT_CLR: *mut u32 = 0xd000_0018 as *mut u32;

const LED: u32 = 25;

struct PinDriver {
    pin: u32
}

impl PinDriver {
    pub fn new(pin: u32) -> PinDriver {
        let gpio_ctrl = (GPIOX_CTRL + 8 * pin) as *mut u32;
        unsafe {
            write_volatile(gpio_ctrl, 5);
            write_volatile(GPIO_OE_SET, 1 << pin);
        };
        PinDriver { pin }
    }
}

impl ErrorType for PinDriver {
    type Error = Infallible;
}

impl OutputPin for PinDriver {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { write_volatile(GPIO_OUT_CLR, 1 << self.pin) }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { write_volatile(GPIO_OUT_SET, 1 << self.pin) }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    unsafe {
        write_volatile((RESET + CLR) as *mut u32, 1 << 5);
        while read_volatile(RESET_DONE as *const u32) & (1 << 5) == 0 {}
    }

    let mut led = PinDriver::new(LED);

    let mut value = 1;
    loop {
        value = 1 - value;
        match value {
            0 => led.set_low(),
            _ => led.set_high(),
        }.unwrap();
        
        for _ in 0..50000 {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
