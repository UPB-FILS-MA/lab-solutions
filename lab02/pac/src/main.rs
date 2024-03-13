#![no_std]
#![no_main]

use core::arch::asm;
use core::convert::Infallible;
use core::panic::PanicInfo;

use cortex_m_rt::entry;
use embedded_hal::digital::{ErrorType, OutputPin};
use rp2040_pac::Peripherals;

const LED: usize = 25;

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

struct PinDriver<'a> {
    pin: usize,
    peripherals: &'a Peripherals,
}

impl<'a> PinDriver<'a> {
    pub fn new(peripherals: &'a Peripherals, pin: usize) -> PinDriver<'a> {
        peripherals
            .IO_BANK0
            .gpio(pin)
            .gpio_ctrl()
            .modify(|_, w| unsafe { w.funcsel().bits(5) });

        let sio = &peripherals.SIO;
        sio.gpio_oe_set().write(|w| unsafe { w.bits(1 << LED) });

        PinDriver { peripherals, pin }
    }
}

impl ErrorType for PinDriver<'_> {
    type Error = Infallible;
}

impl<'a> OutputPin for PinDriver<'a> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.peripherals
            .SIO
            .gpio_out_clr()
            .write(|w| unsafe { w.bits(1 << self.pin) });
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.peripherals
            .SIO
            .gpio_out_set()
            .write(|w| unsafe { w.bits(1 << self.pin) });
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    peripherals
        .RESETS
        .reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });
    while peripherals.RESETS.reset_done().read().bits() & (1 << 5) == 0 {}

    let mut led = PinDriver::new(&peripherals, LED);

    let mut value = 1;
    loop {
        value = 1 - value;
        match value {
            0 => led.set_low(),
            _ => led.set_high(),
        }
        .unwrap();

        for _ in 0..50000 {
            unsafe { asm!("nop") }
        }
    }
}

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}
