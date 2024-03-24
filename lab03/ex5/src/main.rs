#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;

// TODO 6 - register a new task that receives the LED and button pins as parameters
//        - loop to wait for the button press and toggle the LED

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // TODO 1 - initialize the device

    // TODO 2 - initialize the button's and LED2's pin

    // TODO 3 - spawn the task that waits for the button press

    // TODO 4 - init LED1's pin

    loop {
        // delete this otherwise it will panic
        todo!()

        // TODO 5 - toggle LED1

        // TODO 5 - sleep
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
