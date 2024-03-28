#![no_std]
#![no_main]
#![allow(unused_imports, dead_code)]

use core::panic::PanicInfo;

use embassy_executor::Spawner;

// GPIO
use embassy_rp::gpio::{Input, Pull};

// PWM
use embassy_rp::pwm::{Config as PwmConfig, Pwm};

#[derive(PartialEq, Copy, Clone)]
enum LedColor {
    Red,
    Yellow,
    Blue,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // TODO 1: Initialize peripherals

    // TODO 2: Create configuration for red LED

    // TODO 3: Create configuration for green LED

    // TODO 4: Create configuration for blue LED

    // TODO 5: Initialize PWM for red LED

    // TODO 6: Initialize PWM for green LED

    // TODO 7: Initialize PWM for blue LED

    // TODO 8: Initialize button

    // Variable for keeping track of current color
    let color: LedColor = LedColor::Red;

    loop {
        // TODO 9: Wait for button press

        // TODO 10: Check what the current color is and change configurations of PWMs to match next color
        match color {
            LedColor::Red => {
                // to do
            }
            LedColor::Yellow => {
                // to do
            }
            LedColor::Blue => {
                // to do
            }
        }
        // TODO 11: Set new configurations for each PWM

        // TODO 12: Modify variable that keeps track of color
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
