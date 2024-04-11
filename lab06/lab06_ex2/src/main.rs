#![no_std]
#![no_main]

use core::panic::PanicInfo;
use embassy_executor::Spawner;

// GPIO
use embassy_rp::gpio::{Level, Output};

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

// TODO: Change the code to read the value of the `id` register
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // SPI configs
    let mut bmp280_config = spi::Config::default();
    bmp280_config.frequency = 2_000_000;

    // SPI pins
    let miso = peripherals.PIN_16;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;

    // SPI definition
    let mut spi = Spi::new(
        peripherals.SPI0,
        clk,
        mosi,
        miso,
        peripherals.DMA_CH0,
        peripherals.DMA_CH1,
        bmp280_config.clone(),
    );

    // CS pin
    let mut bmp280_cs = Output::new(peripherals.PIN_3, Level::High);

    const REG_ADDR: u8 = 0x00;

    let tx_buf = [(1 << 7) | REG_ADDR, 0x00];
    let mut rx_buf = [0u8; 2];

    bmp280_cs.set_low();
    spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
    bmp280_cs.set_high();

    let register_value = rx_buf[1];

    loop {
        Timer::after_millis(1000).await;
        info!("Register: {register_value}");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
