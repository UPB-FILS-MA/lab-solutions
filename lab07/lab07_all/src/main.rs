#![no_std]
#![no_main]
#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

use core::panic::PanicInfo;
use embassy_executor::Spawner;

use embassy_rp::gpio::{Input, Level, Output, Pin, Pull};

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler as USBInterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};
use log::info;

use embassy_time::Timer;

// I2C
use embassy_rp::i2c::{Config as I2cConfig, I2c, InterruptHandler as I2CInterruptHandler};
use embassy_rp::peripherals::I2C0;
use embedded_hal_async::i2c::{Error, I2c as _};

use core::cell::RefCell;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::spi;
use embassy_rp::spi::{Async, Blocking, Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;

use core::fmt::Write;
use embassy_time::Delay;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::renderer::CharacterStyle;
use embedded_graphics::text::Text;
use heapless::String;
use lab07_all::SPIDeviceInterface;
use st7789::{Orientation, ST7789};

bind_interrupts!(struct Irqs {
    // Use for the serial over USB driver
    USBCTRL_IRQ => USBInterruptHandler<USB>;
    I2C0_IRQ => I2CInterruptHandler<I2C0>;
});

const DISPLAY_FREQ: u32 = 64_000_000;

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

    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;

    // Display SPI pins
    let miso = peripherals.PIN_4;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;

    // Display SPI
    let mut spi_display: Spi<'_, _, Blocking> =
        Spi::new_blocking(peripherals.SPI0, clk, mosi, miso, display_config.clone());
    // SPI bus for display
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi_display));

    let mut display_cs = Output::new(peripherals.PIN_17, Level::High);

    // Display SPI device initialization
    let display_spi = SpiDeviceWithConfig::new(&spi_bus, display_cs, display_config);

    // Other display pins
    let rst = peripherals.PIN_0;
    let dc = peripherals.PIN_16;
    let dc = Output::new(dc, Level::Low);
    let rst = Output::new(rst, Level::Low);
    let di = SPIDeviceInterface::new(display_spi, dc);

    // Init ST7789 LCD
    let mut display = ST7789::new(di, rst, 240, 240);
    display.init(&mut Delay).unwrap();
    display.set_orientation(Orientation::Portrait).unwrap();
    display.clear(Rgb565::BLACK).unwrap();

    // Define style
    let mut style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
    style.set_background_color(Some(Rgb565::BLACK));

    // EXERCISE 2 --------------------
    // TODO 1: Declare SDA and SCL pins

    // TODO 2: Define async I2C

    // TODO 3: Define I2C address of the BMP280 (from lab or from datasheet)
    const BMP280_ADDR: u16 = 0x00;

    // TODO 4: Define ID register address (from last lab or from datasheet)
    const REG_ADDR_ID: u8 = 0x00;
    // EXERCISE 3 --------------------
    // TODO 9: Define CTRL_MEAS register address
    const REG_ADDR_CTRL_MEAS: u8 = 0x00;

    // TODO 12: Define PRESS register address
    const REG_ADDR_PRESS_MSB: u8 = 0x00;

    // TODO 17: Define TEMP register address
    const REG_ADDR_TEMP_MSB: u8 = 0x00;

    // TODO 5: Define TX buffer that will be sent to the sensor
    //         This should be initialized with one element: the address of the register we want to read

    // TODO 6: Define RX buffer that will store the value received from the sensor
    //         This can be initially set to a buffer with one empty value (0)
    //         This will store the value of the register we read from the sensor

    // TODO 7: Write the TX buffer and read to the RX buffer
    //         You can use the `write_read_async` function here, and provide it with the I2C address of the BMP280

    // TODO 8: Get the value of the ID register from the RX buffer
    // END EXERCISE 2 ------------------
    let id = 0;

    loop {
        Timer::after_millis(1000).await;
        info!("ID of BMP280: {id}");
        // TODO 10: Define TX buffer for configuring the sensor (for writing to the `ctrl_meas` register)
        //          The contents of the buffer are similar to what we used in the last lab with SPI

        // TODO 11: Write this buffer to the sensor (`write`)

        // TODO 13: Define TX and RX buffers for reading pressure registers
        //          Hint: The RX buffer should have 3 elements, since we want to
        //                read press_msb, press_lsb and press_xlsb

        // TODO 14: Read the three pressure register values from the sensor (just like you did with the `id`)

        // TODO 15: Compute the raw pressure value from the three register values
        let pressure_raw: u32 = 0; // modify

        // TODO 16: Print the raw pressure value

        info!("Raw pressure reading: {pressure_raw}");

        // TODO 18: Define TX and RX buffers for reading temperature registers

        // TODO 19: Read the three temperature register values from the sensor

        // TODO 20: Compute the raw temperature value from the three register values

        let temperature_raw: u32 = 0; // modify

        // TODO 21: Get the actual temperature value (in Celsius), using the provided `calculate_temperature`
        //          function
        let temperature: i32 = 0; // modify

        // TODO 22: Print the actual temperature value

        info!("Temperature reading (Celsius): {temperature}");

        // END EXERCISE 3 -------------------------

        // EXERCISE 4
        // TODO 23: Print the raw pressure and the actual temperature to the screen
        //          Hint: The temperature value returned by the `calculate_temperature` function is 100 * temperature
        //                Print the correct value to the screen, as a rational number (example: 24.50 degrees)
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
