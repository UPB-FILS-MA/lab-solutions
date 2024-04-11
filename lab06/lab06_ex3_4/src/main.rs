#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use embassy_executor::Spawner;

// GPIO
use embassy_rp::gpio::{Level, Output};
use embassy_rp::pwm::{Config as PwmConfig, Pwm};

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};
use log::info;

use embassy_rp::spi;
use embassy_rp::spi::Spi;
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    // Use for the serial over USB driver
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

// The formula for calculating the actual temperature value (in Celsius) from the raw value
fn calculate_temperature(temperature_raw: u32) -> i32 {
    let var1: i32 = ((temperature_raw as i32 >> 3) - (27504 << 1)) * (26435 >> 11);
    let var2: i32 = ((temperature_raw as i32 >> 4) - 27504)
        * (((temperature_raw as i32 >> 4) - 27504) >> 12)
        * (-1000 >> 14);
    ((var1 + var2) * 5 + 128) >> 8
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // EXERCISE 3 start
    // TODO 1: Initialize the BMP280 SPI config

    // TODO 2: Initialize the MISO, MOSI and CLK pins

    // TODO 3: Create SPI instance

    // TODO 4: Initialize the CS pin

    // EXERCISE 4 start
    // TODO 19: Create a PWM device for the buzzer
    // Hint: top is frequency of the sound, compare is the intensity of the sound

    // TODO 5: Initialize address for the `ctrl_meas` register (use the datasheet to find the address)

    // TODO 11: Initialize address for the `press` register

    // TODO 18: Do the same thing for the temperature value (repeat TODOs 5 to 18, but with different addresses)
    // EXERCISE 3 end

    loop {
        Timer::after_millis(1000).await;

        // TODO 6: Create buffer that will be transmitted over the `MOSI` line
        //         This buffer should contain the control byte for "write to ctrl_meas" on the first position,
        //         and the value to be written to this register on the second position
        //         (oversampling value for temperature, oversampling value for pressure and power mode)
        // Hint: The oversampling values you choose are not that important, they just change the `xlsb` register,
        //       or make the resolution of the ADC measurement better.
        //       What is IMPORTANT is that you change those values to be different than 0!

        // TODO 7: Create buffer that will store the values received from the `MISO` line
        // Hint: This buffer can be initialized with empty values.

        // TODO 12: Create buffer to be sent over `MOSI`.
        //          This buffer should contain the control byte for "read from press" on the first position,
        //          and two other empty values. The empty values will be sent when the sub sends back the data.

        // TODO 13: Create buffer for storing values received from `MISO`
        // Hint: We need a buffer with a size of 4, because we can receive the msb, lsb and xlsb of the register one after the other.

        // TODO 8: Activate the sensor (set CS to low)

        // TODO 9: Make the transfer

        // TODO 14: Make the next transfer (you can do it right after the first one, before you disable the sensor)

        // TODO 10: Deactivate the sensor (set CS to high)

        // TODO 15: Extract the press_msb, press_lsb and press_xlsb from the received buffer

        // TODO 16: Compute the raw pressure value (use a u32, since the register size is 20 bits)

        // TODO 17: Print the pressure value over the serial

        // TODO 20: Calculate the actual temperature in Celsius (use the `calculate_temperature` function)

        // TODO 21: Check if the actual temperature is over a certain value
        //          If it is, change the PWM configuration of the buzzer to make it play a sound
        //          If it isn't, change the configuration back so that the buzzer is silent
        // EXERCISE 4 end
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
