#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};

// TODO 1 - set this function as the main embassy-rs task
//          delete #[allow(unused)]
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // TODO 2 - init the RP2040
    let p = embassy_rp::init(Default::default());

    // TODO 3 - init the GPIO pin used for the LED
    let mut led = Output::new(p.PIN_25, Level::Low);

    let mut value = 1;
    loop {
        value = 1 - value;

        // TODO 4 - write the value to the LED
        match value {
            0 => led.set_low(),
            _ => led.set_high(),
        }

        // TODO 5 - sleep
        Timer::after_millis(200).await;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
