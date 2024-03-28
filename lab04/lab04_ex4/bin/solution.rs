#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;

use embassy_time::Timer;
use log::info;

// USB driver
use embassy_rp::{bind_interrupts, peripherals::USB};
use embassy_rp::usb::{Driver, InterruptHandler};

// ADC
use embassy_rp::adc::{Adc, InterruptHandler as InterruptHandlerAdc, Config as AdcConfig, Channel};

// GPIO
use embassy_rp::gpio::Pull;

// PWM
use embassy_rp::pwm::{Pwm, Config as PwmConfig};

// TODO 4: Bind ADC interrupt
bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    ADC_IRQ_FIFO => InterruptHandlerAdc;
});

// Task used by the serial port driver over USB
#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // TODO 1: Initialize peripherals
    let peripherals = embassy_rp::init(Default::default());

    // TODO 2: Create initial config for PWM
    let mut config: PwmConfig = Default::default();
    // Set top value (or value at which PWM counter will overflow)
    config.top = 0x8000; // in HEX
    // Set compare value (value at which the signal will change from 1 to 0)
    config.compare_a = config.top / 2;

    // TODO 3: Create PWM
    let mut pwm = Pwm::new_output_a(
        peripherals.PWM_CH0, 
        peripherals.PIN_0, 
        config.clone()
    );

    // TODO 5: Create ADC
    let mut adc = Adc::new(peripherals.ADC, Irqs, AdcConfig::default());

    // TODO 6: Initialize photoresistor pin
    let mut light_sensor = Channel::new_pin(peripherals.PIN_26, Pull::None);

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    loop {
        // TODO 7: Read the value of ADC
        let level = adc.read(&mut light_sensor).await.unwrap();
        info!("Light sensor reading: {}", level);
        // TODO 8: Set the duty cycle according to the value of the photoresistor (the brighter the room is, the less bright the led is)
        config.compare_a = config.top / level * 10;
        pwm.set_config(&config);
        // TODO 9: Wait a bit before reading another value
        Timer::after_secs(1).await;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
