#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use embassy_executor::Spawner;

// GPIO
use embassy_rp::gpio::{Input, Level, Output, Pin, Pull};
use embassy_rp::peripherals::{PIN_12, PIN_13};
use embassy_rp::pwm::{Config as PwmConfig, Pwm};

// Channel
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Sender};

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};
use embedded_hal::spi::{SpiBus, SpiDevice};
use log::info;

use core::cell::RefCell;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::spi;
use embassy_rp::spi::{Async, Blocking, Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::Timer;

use core::fmt::Write;
use embassy_time::Delay;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::renderer::CharacterStyle;
use embedded_graphics::text::Text;
use heapless::String;
use lab06_ex5::SPIDeviceInterface;
use st7789::{Orientation, ST7789};

bind_interrupts!(struct Irqs {
    // Use for the serial over USB driver
    USBCTRL_IRQ => InterruptHandler<USB>;
});

const DISPLAY_FREQ: u32 = 64_000_000;

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

// The formula for calculating the actual temperature value (in Celsius) from the raw value
fn calculate_temperature(temperature_adc: u32) -> i32 {
    let var1: i32 = ((temperature_adc as i32 >> 3) - (27504 << 1)) * (26435 >> 11);
    let var2: i32 = ((temperature_adc as i32 >> 4) - 27504)
        * (((temperature_adc as i32 >> 4) - 27504) >> 12)
        * (-1000 >> 14);
    ((var1 + var2) * 5 + 128) >> 8
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // TODO 1: Initialize the BMP280 SPI config

    // Display SPI config
    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;

    // TODO 2: Initialize MISO, MOSI and CLK pins for BMP280 (don't forget to change the circuit too!)

    // Display SPI pins
    let miso = peripherals.PIN_4;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;

    // TODO 3: Initialize SPI for the BMP280 on SPI1

    // Display SPI on SPI0
    let spi_display: Spi<'_, _, Blocking> =
        Spi::new_blocking(peripherals.SPI0, clk, mosi, miso, display_config.clone());
    // SPI bus for display
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi_display));

    // TODO 4: Initialize CS pin for BMP280
    let display_cs = Output::new(peripherals.PIN_17, Level::High);

    // Display SPI device initialization
    let display_spi = SpiDeviceWithConfig::new(&spi_bus, display_cs, display_config);

    // TODO 6: Create a bmp280 SpiDeviceWithConfig, similar to the Display device
    //         This is a driver that we can use do make the two SPI devices share the same SPI channel
    // Warning: You will need to change the physical wiring so that the sensor uses the same SPI pins as the screen

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

    // TODO 5: Retrieve the temperature data from the sensor, just like you did for the previous exercise

    // TODO 7: Modify the transfer function to use the bmp280_spi SPI device you initialized, instead of the old SPI
    // Hint: You don't need to manually set the CS to high and low anymore, the driver does that for us!
    //       All you need to do is transfer the data.

    loop {
        Timer::after_millis(1000).await;

        let temperature = 0; // This will be your temperature value in Celsius

        let mut text = String::<64>::new();
        write!(text, "Temperature: {temperature}").unwrap();

        Text::new(&text, Point::new(40, 110), style)
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
