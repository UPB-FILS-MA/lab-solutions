#![no_std]
#![no_main]

use core::arch::asm;
use core::cell::Cell;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

use cortex_m_rt::{entry, exception, ExceptionFrame};
use critical_section::Mutex;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use rp2040_hal::gpio::bank0::{Gpio0, Gpio12, Gpio25};
use rp2040_hal::gpio::{
    FunctionSioInput, FunctionSioOutput, Interrupt, Pin, Pins, PullNone, PullUp,
};
use rp2040_hal::Sio;
use rp2040_pac::{interrupt, Peripherals};

// TODO 1 - add the RP2040 bootloader
#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

/* Exercise 4 */
// TODO 13 - define a structure that store two pins:
//           - the switch_a pin
//           - the LED1 pin
struct GlobalData {
    led: Pin<Gpio25, FunctionSioOutput, PullNone>,
    switch_a: Pin<Gpio12, FunctionSioInput, PullUp>,
}

// TODO 14 - define a sharable static variable that stores a type of the data structure

static GLOBAL_DATA: Mutex<Cell<Option<GlobalData>>> = Mutex::new(Cell::new(None));

// TODO 15 - register an interrupt handler for IO_IRQ_BANK0
//           inside the critical section, toggle the pin and clear the interrupt
#[interrupt]
unsafe fn IO_IRQ_BANK0() {
    critical_section::with(|cs| {
        let mut data = GLOBAL_DATA.borrow(cs).take();

        if let Some(ref mut data) = data {
            let _ = data.led.toggle();
            data.switch_a.clear_interrupt(Interrupt::EdgeLow);
        }

        GLOBAL_DATA.borrow(cs).replace(data);
    });
}

/***************/

#[entry]
fn main() -> ! {
    // TODO 2 - take the peripherals
    let mut peripherals = Peripherals::take().unwrap();

    // TODO 3 - enable the IO_BANK0 peripheral
    let reset = &peripherals.RESETS;
    reset
        .reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });
    while reset.reset_done().read().bits() & (1 << 5) == 0 {}

    // TODO 4 - initialize SIO with Sio::new()
    let sio = Sio::new(peripherals.SIO);

    // TODO 5 - initialize Pins with Pins::new()
    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // TODO 6 - initialize GPIO for LED 2
    //          use Pin<GpioX, Function, PullType>
    //          with `pins.reconfigure()`
    let mut led: Pin<Gpio0, FunctionSioOutput, PullNone> = pins.gpio0.reconfigure();

    /* Exercise 3 */

    // TODO 10 - trigger a hard fault
    // define an invalid memory address
    // const INVALID_ADDRESS: *mut u32 = 0xf000_0000 as *mut u32;

    // write to it
    // unsafe {
    //     // this triggers a hard fault
    //     write_volatile(INVALID_ADDRESS, 0);
    // }

    /**************/

    /* Exercise 4 */

    // TODO 16 - initialize the button

    let switch_a: Pin<Gpio12, FunctionSioInput, PullUp> = pins.gpio12.reconfigure();

    // TODO 17 - enable the interrupt for the button for `Interrupt:EdgeLow`
    switch_a.set_interrupt_enabled(Interrupt::EdgeLow, true);

    // TODO 18 - initialize the pin for LED2
    let led2: Pin<Gpio25, FunctionSioOutput, PullNone> = pins.gpio25.reconfigure();

    // TODO 18 - initialize the structure to store the led and the button

    let local_data = GlobalData {
        led: led2,
        switch_a,
    };

    // TODO 19 - store the local structure in the global variable
    critical_section::with(|cs| {
        GLOBAL_DATA.borrow(cs).replace(Some(local_data));
    });

    // TODO 20 - unmask the NVIC IO_Bank0 interrupt
    unsafe {
        rp2040_pac::NVIC::unmask(rp2040_pac::Interrupt::IO_IRQ_BANK0);
    }

    /*************/

    let mut value = 1;
    // for _ in 1..10 {
    loop {
        value = 1 - value;
        // TODO 7 - write the value to the LED
        match value {
            0 => led.set_low(),
            _ => led.set_high(),
        }
        .unwrap();

        // TODO 8 - sleep
        for _ in 0..50000 {
            unsafe { asm!("nop") }
        }
    }

    /* Exercise 3 */

    // TODO 12 - delete the generated fault
    //           use a for instead of a loop above
    //           panic! here
    // panic!();

    /**************/
}

/* Exercise 3 */

// TODO 9 - register the hard fault handler
//          use the code from lab02 - bare metal to blink LED1
#[exception]
unsafe fn HardFault(_frame: &ExceptionFrame) -> ! {
    panic!()
}

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    // TODO 11 - move the hard fault handler code here and trigger
    //           a panic! in the hard fault handler
    // reset IO Bank0
    const RESET: u32 = 0x4000_c000;
    const CLR: u32 = 0x3000;

    const RESET_DONE: u32 = 0x4000_c008;

    const GPIOX_CTRL: u32 = 0x4001_4004;
    const GPIO_OE_SET: *mut u32 = 0xd000_0024 as *mut u32;
    const GPIO_OUT_SET: *mut u32 = 0xd000_0014 as *mut u32;
    const GPIO_OUT_CLR: *mut u32 = 0xd000_0018 as *mut u32;

    const LED: u32 = 25;

    unsafe {
        write_volatile((RESET + CLR) as *mut u32, 1 << 5);
        while read_volatile(RESET_DONE as *const u32) & (1 << 5) == 0 {}
    }

    let gpio_ctrl = (GPIOX_CTRL + 8 * LED) as *mut u32;
    unsafe {
        write_volatile(gpio_ctrl, 5);
    };

    unsafe {
        write_volatile(GPIO_OE_SET, 1 << LED);
    };

    let mut value = 0;

    loop {
        value = 1 - value;
        let reg = match value {
            0 => GPIO_OUT_CLR,
            _ => GPIO_OUT_SET,
        };

        unsafe { write_volatile(reg, 1 << LED) }

        // TODO 9 - sleep
        for _ in 0..5000 {
            unsafe { asm!("nop") }
        }
    }
}
