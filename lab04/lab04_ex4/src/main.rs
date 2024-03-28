#![no_std]
#![no_main]
#![allow(unused_imports, unused_variables)]

use core::panic::PanicInfo;

use embassy_executor::Spawner;

use embassy_time::Timer;
use log::info;

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};

// ADC
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig, InterruptHandler as InterruptHandlerAdc};

// GPIO
use embassy_rp::gpio::Pull;

// PWM
use embassy_rp::pwm::{Config as PwmConfig, Pwm};

// TODO 4: Bind ADC interrupt

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // TODO 1: Initialize peripherals

    // TODO 2: Create initial config for PWM

    // TODO 3: Create PWM

    // TODO 5: Create ADC

    // TODO 6: Initialize photoresistor pin

    loop {
        // delete this otherwise it will panic
        todo!();
        // TODO 7: Read the value of ADC

        // TODO 8: Set the duty cycle according to the value of the photoresistor (the brighter the room is, the less bright the led is)

        // TODO 9: Wait a bit before reading another value
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
