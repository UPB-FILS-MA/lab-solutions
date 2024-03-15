#![no_main]
#![no_std]

use core::arch::asm;
use core::convert::Infallible;
use core::panic::PanicInfo;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m_rt::ExceptionFrame;
use embedded_hal::digital::ErrorType;
use embedded_hal::digital::OutputPin;

// TODO 1 - add the RP2040 bootloader
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

/* Exercise 2 */

struct PinDriver {
    // TODO 11 - define a function that takes as a parameter a pin number:
    //           - set the LED pin the SIO function in IO_BANK0
    //           - returns a `PinDriver` structure`
    pin: u32,
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

// The driver will never fail
impl ErrorType for PinDriver {
    type Error = Infallible;
}

impl OutputPin for PinDriver {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // TODO 12 - set the pin low
        unsafe { write_volatile(GPIO_OUT_CLR, 1 << self.pin) }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        // TODO 13 - set the pin high
        unsafe { write_volatile(GPIO_OUT_SET, 1 << self.pin) }
        Ok(())
    }
}

#[exception]
unsafe fn HardFault(_frame: &ExceptionFrame) -> ! {
    loop {}
}

#[exception]
unsafe fn SysTick() {
    // execute at a fixed interval
}

// TODO 3 - make the main function the entry point
//          delete #[allow(unused)]
#[entry]
fn main() -> ! {
    // TODO 4 - enable the IO_BANK0 peripheral
    unsafe {
        write_volatile((RESET + CLR) as *mut u32, 1 << 5);
        while read_volatile(RESET_DONE as *const u32) & (1 << 5) == 0 {}
    }

    // TODO 5 - set the LED pin the SIO function in IO_BANK0
    // let gpio_ctrl = (GPIOX_CTRL + 8 * pin) as *mut u32;
    // unsafe {
    //     write_volatile(gpio_ctrl, 5);
    // };

    // TODO 6 - set the LED pin as output in SIO
    // unsafe {
    //     write_volatile(GPIO_OE_SET, 1 << pin);
    // };

    // TODO 7 - set the value of LED to HIGH
    // unsafe { write_volatile(GPIO_OUT_SET, 1 << self.pin) }

    let mut led = PinDriver::new(LED);

    /* Exercise 3 */
    let mut value = 1;
    loop {
        value = 1 - value;
        // TODO 8 - write the value to the LED
        // let reg = match value {
        //     0 => GPIO_OUT_CLR,
        //     _ => GPIO_OUT_SET,
        // };

        // unsafe { write_volatile(reg, 1 << self.pin) }

        // TODO 14 - use the PinDriver
        match value {
            0 => led.set_low(),
            _ => led.set_high(),
        }
        .unwrap();

        // TODO 9 - sleep
        for _ in 0..50000 {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
