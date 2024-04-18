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

    // Display SPI on SPI0
    let mut spi_display: Spi<'_, _, Blocking> =
        Spi::new_blocking(peripherals.SPI0, clk, mosi, miso, display_config.clone());
    // SPI bus for display
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi_display));

    // TODO 4: Initialize CS pin for BMP280
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
    let sda = peripherals.PIN_20;
    let scl = peripherals.PIN_21;

    // TODO 2: Define async I2C
    let mut i2c = I2c::new_async(peripherals.I2C0, scl, sda, Irqs, I2cConfig::default());

    // TODO 3: Define I2C address of the BMP280 (from lab or from datasheet)
    const BMP280_ADDR: u16 = 0x76;

    // TODO 4: Define ID register address (from last lab or from datasheet)
    const REG_ADDR_ID: u8 = 0xD0;
    // EXERCISE 3 --------------------
    // TODO 9: Define CTRL_MEAS register address
    const REG_ADDR_CTRL_MEAS: u8 = 0xf4;

    // TODO 12: Define PRESS register address
    const REG_ADDR_PRESS_MSB: u8 = 0xf7;

    // TODO 17: Define TEMP register address
    const REG_ADDR_TEMP_MSB: u8 = 0xfa;

    // TODO 5: Define TX buffer that will be sent to the sensor
    //         This will contain the address of the register we want to read
    let tx_buf_id = [REG_ADDR_ID];
    // TODO 6: Define RX buffer that will store the value received from the sensor
    //         This can be initially set to 0, and it will then store the value of the register we read from the sensor
    let mut rx_buf_id = [0x00u8];
    // TODO 7: Write the TX buffer and read to the RX buffer
    //         You can use the `write_read_async` function here, and provide it with the I2C address of the BMP280
    i2c.write_read(BMP280_ADDR, &tx_buf_id, &mut rx_buf_id)
        .await
        .unwrap();

    // TODO 8: Get the value of the ID register from the RX buffer
    // END EXERCISE 2 ------------------
    let id = 0;

    loop {
        Timer::after_millis(1000).await;
        info!("ID of BMP280: {id}");
        // TODO 10: Define TX buffer for configuring the sensor (for writing to the `ctrl_meas` register)
        let tx_buf_ctrl_meas = [REG_ADDR_CTRL_MEAS, 0b001_001_11];

        // TODO 11: Write this buffer to the sensor
        i2c.write(BMP280_ADDR, &tx_buf_ctrl_meas).await.unwrap();

        // TODO 13: Define TX and RX buffers for reading pressure registers
        //          Hint: The RX buffer should have 3 elements, since we want to
        //                read press_msb, press_lsb and press_xlsb
        let tx_buf_press = [REG_ADDR_PRESS_MSB];
        let mut rx_buf_press = [0x00u8; 3];
        // TODO 14: Read the three pressure register values from the sensor (just like you did with the `id`)
        i2c.write_read(BMP280_ADDR, &tx_buf_press, &mut rx_buf_press)
            .await
            .unwrap();

        // TODO 15: Compute the raw pressure value from the three register values
        let press_msb = rx_buf_press[0] as u32;
        let press_lsb = rx_buf_press[1] as u32;
        let press_xlsb = rx_buf_press[2] as u32;

        let pressure_raw: u32 = (press_msb << 12) + (press_lsb << 4) + (press_xlsb >> 4);

        // TODO 16: Print the raw pressure value
        info!("Raw pressure reading: {pressure_raw}");

        // TODO 18: Define TX and RX buffers for reading temperature registers
        i2c.write(BMP280_ADDR, &tx_buf_ctrl_meas).await.unwrap();
        let tx_buf_temp = [REG_ADDR_TEMP_MSB];
        let mut rx_buf_temp = [0x00u8; 3];

        // TODO 19: Read the three temperature register values from the sensor
        i2c.write_read(BMP280_ADDR, &tx_buf_temp, &mut rx_buf_temp)
            .await
            .unwrap();

        // TODO 20: Compute the raw temperature value from the three register values
        let temp_msb = rx_buf_temp[0] as u32;
        let temp_lsb = rx_buf_temp[1] as u32;
        let temp_xlsb = rx_buf_temp[2] as u32;

        let temperature_raw: u32 = (temp_msb << 12) + (temp_lsb << 4) + (temp_xlsb >> 4);

        // TODO 21: Get the actual temperature value (in Celsius), using the provided function
        let temperature = calculate_temperature(temperature_raw);
        let temperature_int = temperature / 100;
        let temperature_frac = temperature % 100;

        // TODO 22: Print the actual temperature value
        info!("Temperature (in Celsius): {temperature_int}.{temperature_frac}");

        // END EXERCISE 3 -------------------------

        let mut text_temp = String::<64>::new();
        let mut text_press = String::<64>::new();
        write!(
            text_temp,
            "Temperature: {temperature_int}.{temperature_frac}"
        )
        .unwrap();

        Text::new(&text_temp, Point::new(40, 110), style)
            .draw(&mut display)
            .unwrap();

        write!(text_press, "Raw pressure: {pressure_raw}").unwrap();

        Text::new(&text_press, Point::new(40, 150), style)
            .draw(&mut display)
            .unwrap();

        // Small delay for yielding
        Timer::after_millis(1).await;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
