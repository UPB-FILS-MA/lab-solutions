#![no_std]
#![no_main]

use core::panic::PanicInfo;
use embassy_executor::Spawner;
use embassy_time::Timer;
use log::info;

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};

// ADC
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig, InterruptHandler as AdcInterruptHandler};
// GPIO
use embassy_rp::gpio::Pull;

// TODO 2: Bind the ADC_IRQ_FIFO interrupt (be careful with the import names)
bind_interrupts!(struct Irqs {
    // Use for the serial over USB driver
    USBCTRL_IRQ => InterruptHandler<USB>;
    ADC_IRQ_FIFO => AdcInterruptHandler;
});

// The task used by the serial port driver over USB
#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // ---- Exercise 3 ----
    // TODO 1: Initialize peripherals
    let peripherals = embassy_rp::init(Default::default());

    // TODO 3: Create ADC
    let mut adc = Adc::new(peripherals.ADC, Irqs, AdcConfig::default());

    // TODO 4: Initialize photoresistor pin on ADC0
    let mut light_sensor = Channel::new_pin(peripherals.PIN_26, Pull::None);

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    loop {
        // TODO 5: Read a value from the ADC
        let level = adc.read(&mut light_sensor).await.unwrap();
        // TODO 6: Print the value to the console (over serial port)
        info!("Light sensor reading: {}", level);
        // TODO 7: Wait a while before reading again
        Timer::after_secs(1).await;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
