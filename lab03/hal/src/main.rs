#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_rt::entry;

// TODO 1 - add the RP2040 bootloader

/* Exercise 4 */
// TODO 13 - define a structure that store two pins:
//           - the switch_a pin
//           - the LED1 pin

// TODO 14 - define a sharable static variable that stores a type of the data structure

// TODO 15 - register an interrupt handler for IO_IRQ_BANK0
//           inside the critical section, toggle the pin and clear the interrupt
/***************/

#[entry]
fn main() -> ! {
    // TODO 2 - take the peripherals

    // TODO 3 - enable the IO_BANK0 peripheral (lab 02 pac)

    // TODO 4 - initialize SIO with Sio::new()

    // TODO 5 - initialize Pins with Pins::new()

    // TODO 6 - initialize GPIO for LED 2
    //          use Pin<GpioX, Function, PullType>
    //          with `pins.reconfigure()`

    /* Exercise 3 */

    // TODO 10 - trigger a hard fault
    /**************/

    /* Exercise 4 */

    // TODO 16 - initialize the button

    // TODO 17 - enable the interrupt for the button for `Interrupt:EdgeLow`

    // TODO 18 - initialize the pin for LED2

    // TODO 18 - initialize the structure to store the led and the button

    // TODO 19 - store the local structure in the global variable

    // TODO 20 - unmask the NVIC IO_Bank0 interrupt
    /*************/

    let mut value = 1;
    loop {
        value = 1 - value;
        // TODO 7 - write the value to the LED

        // TODO 8 - sleep
    }

    /* Exercise 3 */

    // TODO 12 - delete the generated fault
    //           use a for instead of a loop above
    //           panic! here
    /**************/
}

/* Exercise 3 */

// TODO 9 - register the hard fault handler
//          use the code from lab02 - bare metal to blink LED1

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    // TODO 11 - move the hard fault handler code here and trigger
    //           a panic! in the hard fault handler

    loop {}
}
