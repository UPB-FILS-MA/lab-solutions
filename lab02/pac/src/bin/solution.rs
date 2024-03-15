#![no_std]
#![no_main]

use core::arch::asm;
use core::convert::Infallible;
use core::panic::PanicInfo;

use cortex_m_rt::entry;
use embedded_hal::digital::{ErrorType, OutputPin};
use rp2040_pac::Peripherals;

// TODO 1 - add the RP2040 bootloader
#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const LED: usize = 25;

/* Exercise 2 */
struct PinDriver<'a> {
    // TODO 10 - store the number of the pin
    pin: usize,
    peripherals: &'a Peripherals,
}

impl<'a> PinDriver<'a> {
    // TODO 11 - define a function that takes as a parameter a pin number:
    //           - set the LED pin the SIO function in IO_BANK0
    //           - returns a `PinDriver` structure
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

// The driver will never fail
impl ErrorType for PinDriver<'_> {
    type Error = Infallible;
}

impl<'a> OutputPin for PinDriver<'a> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // TODO 12 - set the pin low
        self.peripherals
            .SIO
            .gpio_out_clr()
            .write(|w| unsafe { w.bits(1 << self.pin) });
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        // TODO 13 - set the pin high
        self.peripherals
            .SIO
            .gpio_out_set()
            .write(|w| unsafe { w.bits(1 << self.pin) });
        Ok(())
    }
}

// TODO 3 - make the main function the entry point
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    // TODO 4 - enable the IO_BANK0 peripheral
    peripherals
        .RESETS
        .reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });
    while peripherals.RESETS.reset_done().read().bits() & (1 << 5) == 0 {}

    // TODO 5 - set the LED pin the SIO function in IO_BANK0
    // peripherals
    //         .IO_BANK0
    //         .gpio(pin)
    //         .gpio_ctrl()
    //         .modify(|_, w| unsafe { w.funcsel().bits(5) });

    // TODO 6 - set the LED pin as output in SIO
    // let sio = &peripherals.SIO;
    // sio.gpio_oe_set().write(|w| unsafe { w.bits(1 << LED) });

    let mut led = PinDriver::new(&peripherals, LED);

    let mut value = 1;
    loop {
        value = 1 - value;

        // TODO 8 - write the value to the LED
        // match value {
        //     0 => self
        //         .peripherals
        //         .SIO
        //         .gpio_out_clr()
        //         .write(|w| unsafe { w.bits(1 << self.pin) }),
        //     _ => self
        //         .peripherals
        //         .SIO
        //         .gpio_out_set()
        //         .write(|w| unsafe { w.bits(1 << self.pin) }),
        // }
        // .unwrap();

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
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}
