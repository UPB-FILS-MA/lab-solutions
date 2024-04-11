#![no_std]
#![no_main]

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
    let mut bmp280_config = spi::Config::default();
    bmp280_config.frequency = 2_000_000;

    // TODO 2: Initialize the MISO, MOSI and CLK pins
    let miso = peripherals.PIN_16;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;

    // TODO 3: Create asynchronous SPI instance
    let mut spi = Spi::new(
        peripherals.SPI0,
        clk,
        mosi,
        miso,
        peripherals.DMA_CH0,
        peripherals.DMA_CH1,
        bmp280_config.clone(),
    );

    // TODO 4: Initialize the CS pin
    let mut bmp280_cs = Output::new(peripherals.PIN_3, Level::High);

    // TODO 19: Create a PWM device for the buzzer
    let mut config_pwm: PwmConfig = Default::default();
    config_pwm.top = 0xFFFF;
    config_pwm.compare_b = 0;

    // Initialize PWM
    let mut buzzer = Pwm::new_output_b(peripherals.PWM_CH0, peripherals.PIN_1, config_pwm.clone());

    // TODO 5: Initialize address for the `ctrl_meas` register (use the datasheet to find the address)
    const REG_ADDR_CTRL_MEAS: u8 = 0xf4;
    // TODO 11: Initialize address for the `press` register
    const REG_ADDR_PRESS_MSB: u8 = 0xf7;
    const REG_ADDR_TEMP_MSB: u8 = 0xfa;

    // TODO 18: Do the same thing for the temperature value (repeat TODOs 5 to 18, but with different addresses)

    loop {
        Timer::after_millis(1000).await;

        // TODO 6: Create buffer that will be transmitted over the `MOSI` line
        //         This buffer should contain the control byte for "write to ctrl_meas" on the first position,
        //         and the value to be written to this register on the second position
        //         (oversampling value for temperature, oversampling value for pressure and power mode)
        // Hint: The oversampling values you choose are not that important, they just change the `xlsb` register,
        //       or make the resolution of the ADC measurement better.
        //       What is IMPORTANT is that you change those values to be different than 0!
        let tx_buf = [!(1 << 7) & REG_ADDR_CTRL_MEAS, 0b001_001_11];
        // TODO 7: Create buffer that will store the values received from the `MISO` line
        // Hint: This buffer can be initialized with empty values.
        let mut rx_buf = [0u8; 2];

        // TODO 12: Create buffer to be sent over `MOSI`.
        //          This buffer should contain the control byte for "read from press" on the first position,
        //          and two other empty values. The empty values will be sent when the sub sends back the data.
        let tx_buf1 = [(1 << 7) | REG_ADDR_PRESS_MSB, 0x00, 0x00, 0x00]; // second value sent is a dummy value
                                                                         // TODO 13: Create buffer for storing values received from `MISO`
                                                                         // Hint: We need a buffer with a size of 4, because we can receive the msb, lsb and xlsb of the register one after the other.
        let mut rx_buf1 = [0u8; 4];

        let tx_buf2 = [(1 << 7) | REG_ADDR_TEMP_MSB, 0x00, 0x00, 0x00]; // second value sent is a dummy value
        let mut rx_buf2 = [0u8; 4];

        // TODO 8: Activate the sensor (set CS to low)
        bmp280_cs.set_low();
        // TODO 9: Make the transfer
        spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
        // TODO 14: Make the next transfer (you can do it right after the first one, before you disable the sensor)
        spi.transfer(&mut rx_buf1, &tx_buf1).await.unwrap();
        // TODO 10: Deactivate the sensor (set CS to high)
        bmp280_cs.set_high();

        bmp280_cs.set_low();
        spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
        spi.transfer(&mut rx_buf2, &tx_buf2).await.unwrap();
        bmp280_cs.set_high();

        // TODO 15: Extract the press_msb, press_lsb and press_xlsb from the received buffer
        let press_msb = rx_buf1[1] as u32;
        let shifted_msb = press_msb << 12;
        let press_lsb = rx_buf1[2] as u32;
        let shifted_lsb = press_lsb << 4;
        let press_xlsb = rx_buf1[3] as u32;
        let shifted_xlsb = press_xlsb >> 4;

        // TODO 16: Compute the raw pressure value (use a u32, since the register size is 20 bits)
        let pressure_raw: u32 = shifted_msb + shifted_lsb + shifted_xlsb;

        // TODO 17: Print the pressure value over the serial
        info!("Pressure {pressure_raw}");

        let temp_msb = rx_buf2[1] as u32;
        let shifted_msb_t = temp_msb << 12;
        let temp_lsb = rx_buf2[2] as u32;
        let shifted_lsb_t = temp_lsb << 4;
        let temp_xlsb = rx_buf2[3] as u32;
        let shifted_xlsb_t = temp_xlsb >> 4;

        let temperature_raw: u32 = shifted_msb_t + shifted_lsb_t + shifted_xlsb_t;

        info!("Temperature {temperature_raw}");

        // TODO 20: Calculate the actual temperature (use the `calculate_temperature function`)
        let temperature = calculate_temperature(temperature_raw);

        info!("Temperature actual {temperature}");

        // TODO 21: Check if the actual temperature is over a certain value
        //          If it is, change the PWM configuration of the buzzer to make it play a sound
        //          If it isn't, change the configuration back so that the buzzer is silent
        if temperature / 100 > 24 {
            config_pwm.compare_b = config_pwm.top / 2;
        } else {
            config_pwm.compare_b = 0;
        }

        buzzer.set_config(&config_pwm);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
