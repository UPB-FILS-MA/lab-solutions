#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::str::from_utf8;

use byte_slice_cast::AsByteSlice;
use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_futures::select;
use embassy_net::tcp::TcpSocket;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{Config, IpAddress, IpEndpoint, Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use heapless::Vec;
use log::{info, warn};
use static_cell::StaticCell;

// USB driver
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, Endpoint, InterruptHandler as USBInterruptHandler};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => USBInterruptHandler<USB>;
    // PIO interrupt for CYW SPI communication
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const WIFI_NETWORK: &str = "TEST";
const WIFI_PASSWORD: &str = "test";

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Start the USB logger driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    let mut button_a = Input::new(peripherals.PIN_12, Pull::Up);

    // Link CYW43 firmware
    let fw = include_bytes!("../../../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../../../cyw43-firmware/43439A0_clm.bin");

    // Init SPI for communication with CYW43
    let pwr = Output::new(peripherals.PIN_23, Level::Low);
    let cs = Output::new(peripherals.PIN_25, Level::High);
    let mut pio = Pio::new(peripherals.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        peripherals.PIN_24,
        peripherals.PIN_29,
        peripherals.DMA_CH0,
    );

    // Start Wi-Fi task
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(wifi_task(runner)).unwrap();

    // Init the device
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());

    // Generate random seed
    let seed = 0x0123_4567_89ab_cdef;

    // Init network stack
    static STACK: StaticCell<Stack<cyw43::NetDriver<'static>>> = StaticCell::new();
    static RESOURCES: StaticCell<StackResources<2>> = StaticCell::new();
    let stack = &*STACK.init(Stack::new(
        net_device,
        config,
        RESOURCES.init(StackResources::<2>::new()),
        seed,
    ));

    // Start network stack task
    spawner.spawn(net_task(stack)).unwrap();

    loop {
        // Join WPA2 access point
        // TODO 2: Modify WIFI_NETWORK and WIFI_PASSWORD if you're connecting to a WPA AP
        //         Use `join_open` instead if you're connecting to an open AP
        match control.join_wpa2(WIFI_NETWORK, WIFI_PASSWORD).await {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status {}", err.status);
            }
        }
    }

    // Wait for DHCP (not necessary when using static IP)
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up {:?}!", stack.config_v4());

    // And now we can use it!

    // TODO 1: Create buffers
    let mut rx_buffer = [0; 4096];
    let mut rx_metadata_buffer = [PacketMetadata::EMPTY; 3];
    let mut tx_buffer = [0; 4096];
    let mut tx_metadata_buffer = [PacketMetadata::EMPTY; 3];

    let mut buf = [0u8; 4096];

    loop {
        // TODO 2: Initialize UDP socket
        let mut socket = UdpSocket::new(
            stack,
            &mut rx_metadata_buffer,
            &mut rx_buffer,
            &mut tx_metadata_buffer,
            &mut tx_buffer,
        );

        info!("Starting server on UDP:1234...");

        // TODO 3: Bind socket to port
        if let Err(e) = socket.bind(1234) {
            warn!("accept error: {:?}", e);
            continue;
        }

        // TODO 4: Wait for button press
        // button_a.wait_for_falling_edge().await;

        // TODO 6: Remove the line in which you wait for the button press
        //         Instead, move the `wait_for_falling_edge` future inside a `select`
        //         The second future inside `select` will be the `recv_from` future

        control.gpio_set(0, true).await;
        let buffer = "Button was pressed!\n".as_bytes();
        match socket
            .send_to(
                &buffer,
                IpEndpoint::new(IpAddress::v4(192, 168, 1, 132), 1234),
            )
            .await
        {
            Ok(()) => {
                info!("sent")
            }
            Err(e) => {
                warn!("send error: {:?}", e);
            }
        }
        control.gpio_set(0, false).await;
        match select::select(button_a.wait_for_falling_edge(), socket.recv_from(&mut buf)).await {
            // TODO 7: Move the code for sending the message to be run when the `button_a` future completes
            select::Either::First(_res_btn) => {
                // TODO 5: Send message to the correct IP and port
            }
            // TODO 8: Handle the `recv_from` future accordingly: the LED on the Pico should light up/turn off
            //         according to the received command
            // Hint: To light the LED on the Pico W, use:
            // ```rust
            // control.gpio_set(0, true).await; // turn on LED
            // control.gpio_set(0, false).await; // turn off LED
            // ```
            select::Either::Second(res_read) => match res_read {
                Ok((n, endpoint)) => {
                    info!(
                        "Received from {:?}: {:?}",
                        endpoint,
                        from_utf8(&buf[..n]).unwrap().trim()
                    );
                    let command = from_utf8(&buf[..n]).unwrap().trim();
                    match command {
                        "led:on" => control.gpio_set(0, true).await,
                        "led:off" => control.gpio_set(0, false).await,
                        _ => {}
                    }
                }
                Err(_) => {
                    info!("An error occurred when receiving the packet!");
                }
            },
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
