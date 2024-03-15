#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};
use embassy_time::Timer;
use log::info;

// Use for the serial over USB driver
bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

// The task used by the serial port driver
// over USB
#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

// TODO 1 - set this function as the main embassy-rs task
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // TODO 2 - initialize button A (pull up)
    let button = Input::new(peripherals.PIN_12, Pull::Up);

    /* Exercise 5 */
    // TODO 4 - initialize the LED
    let mut led = Output::new(peripherals.PIN_25, Level::Low);

    loop {
        // TODO 3 - verify the state of the button
        if button.is_low() {
            //          if it is down (the button is pressed)
            //          - print a message
            info!("The button was pressed");
            //          TODO 5 - Exercise 5 - toggle the LED
            led.toggle();
            //          - sleep
            Timer::after_millis(10).await;
            //          - wait for the button to be released
            while button.is_low() {
                //              - sleep
                Timer::after_millis(10).await;
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
