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
//          delete #[allow(unused)]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    let button = Input::new(peripherals.PIN_12, Pull::Up);
    let mut led = Output::new(peripherals.PIN_25, Level::Low);

    loop {
        if button.is_low() {
            info!("The button was pressed");
            led.toggle();
            while button.is_low() {
                Timer::after_millis(10).await;
            }


        }
        Timer::after_millis(10).await;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
