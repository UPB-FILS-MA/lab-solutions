#![no_std]
#![no_main]

use core::convert::Infallible;
use core::panic::PanicInfo;

use embedded_hal::digital::{ErrorType, OutputPin};
use rp2040_pac::Peripherals;

// TODO 1 - add the RP2040 bootloader

// const LED: usize = TODO 2 - add here the GPIO pin used for the LED;

/* Exercise 2 */

struct PinDriver<'a> {
    // TODO 10 - store the number of the pin
    _peripherals: &'a Peripherals,
}

impl<'a> PinDriver<'a> {
    // TODO 11 - define a function that takes as a parameter a pin number:
    //           - set the LED pin the SIO function in IO_BANK0
    //           - returns a `PinDriver` structure
}

// The driver will never fail
impl ErrorType for PinDriver<'_> {
    type Error = Infallible;
}

impl OutputPin for PinDriver<'_> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // TODO 12 - set the pin low
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        // TODO 13 - set the pin high
        Ok(())
    }
}

// TODO 3 - make the main function the entry point
//          delete #[allow(unused)]
#[allow(unused)]
fn main() -> ! {
    // TODO 4 - enable the IO_BANK0 peripheral

    // TODO 5 - set the LED pin the SIO function in IO_BANK0

    // TODO 6 - set the LED pin as output in SIO

    // TODO 7 - set the value of LED to HIGH

    /* Exercise 3 */
    let mut value = 1;
    loop {
        value = 1 - value;
        // TODO 8 - write the value to the LED

        // TODO 14 - use the PinDriver

        // TODO 9 - sleep
    }
}

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}
