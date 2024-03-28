#![no_std]
#![no_main]

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

static TOP: u16 = 0x8000;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // TODO 1: Initialize peripherals
    let peripherals = embassy_rp::init(Default::default());

    // TODO 2: Create configuration for red LED
    let mut config_red: PwmConfig = Default::default();
    config_red.top = TOP;
    config_red.compare_b = config_red.top;

    // TODO 3: Create configuration for green LED
    let mut config_green: PwmConfig = Default::default();
    config_green.top = TOP;
    config_green.compare_b = 0;

    // TODO 4: Create configuration for blue LED
    let mut config_blue: PwmConfig = Default::default();
    config_blue.top = TOP;
    config_blue.compare_b = 0;

    // TODO 5: Initialize PWM for red LED
    let mut pwm_red = Pwm::new_output_b(peripherals.PWM_CH0, peripherals.PIN_1, config_red.clone());
    // TODO 6: Initialize PWM for green LED
    let mut pwm_green =
        Pwm::new_output_a(peripherals.PWM_CH1, peripherals.PIN_2, config_green.clone());
    // TODO 7: Initialize PWM for blue LED
    let mut pwm_blue =
        Pwm::new_output_a(peripherals.PWM_CH2, peripherals.PIN_4, config_blue.clone());

    // TODO 8: Initialize button
    let mut button = Input::new(peripherals.PIN_12, Pull::Up);

    // Variable for keeping track of current color
    let mut color: LedColor = LedColor::Red;

    loop {
        // TODO 9: Wait for button press
        button.wait_for_falling_edge().await;
        // TODO 10: Check what the current color is and change configurations of PWMs to match next color
        match color {
            LedColor::Red => {
                config_red.compare_b = TOP;
                config_green.compare_a = 0;
                config_blue.compare_a = 0;
                color = LedColor::Yellow;
            }
            LedColor::Yellow => {
                config_red.compare_b = TOP;
                config_green.compare_a = TOP;
                config_blue.compare_a = 0;
                color = LedColor::Blue;
            }
            LedColor::Blue => {
                config_red.compare_b = 0;
                config_green.compare_a = 0;
                config_blue.compare_a = TOP;
                color = LedColor::Red;
            }
        }
        // TODO 11: Set new configurations for each PWM
        pwm_red.set_config(&config_red);
        pwm_green.set_config(&config_green);
        pwm_blue.set_config(&config_blue);
        // TODO 12: Modify variable that keeps track of color
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
