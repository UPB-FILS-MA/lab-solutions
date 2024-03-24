#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{self, Input, Pull},
    peripherals::{PIN_12, PIN_25},
};
use embassy_time::Timer;
use gpio::{Level, Output};

#[embassy_executor::task]
async fn button_pressed(mut led: Output<'static, PIN_25>, mut button: Input<'static, PIN_12>) {
    loop {
        // Wait for button button to be pressed
        button.wait_for_falling_edge().await;
        led.toggle();
        // Do something only if button was pressed
        // (Here should be the logic for your interrupt request handler)
        // Example: turn on the LED
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // TODO 1 - initialize the device
    let peripherals = embassy_rp::init(Default::default());

    // TODO 2 - initialize the button's and LED2's pin
    let button = Input::new(peripherals.PIN_12, Pull::Up);
    let led2 = Output::new(peripherals.PIN_25, Level::Low);

    // TODO 3 - spawn the task that waits for the button press
    spawner.spawn(button_pressed(led2, button)).unwrap();

    // TODO 4 - init LED1's pin
    let mut led = Output::new(peripherals.PIN_0, Level::Low);

    loop {
        // TODO 5 - toggle LED1
        led.toggle();

        // TODO 5 - sleep
        Timer::after_millis(200).await;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
