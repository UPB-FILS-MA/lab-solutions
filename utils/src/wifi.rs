//! Utilities for WiFi chip initialization and management.

/// Creates a tuple comprising a handle to the network device, a control handle and a runner for
/// driving the low level stack and should be invoked on the RP2040 `Peripherals` and the struct that implements the `Binding` for the PIO
/// interrupt to its handler.
///
/// `wifi_init!` initializes the WiFi chip, including loading firmware, setting up SPI communication,
/// and configuring power management.
///
/// # Example
///
/// ```
/// use embassy_executor::Spawner;
/// use embassy_rp::bind_interrupts;
/// use embassy_rp::peripherals::PIO0;
/// use embassy_rp::pio::InterruptHandler;
///
/// // Bind interrupts to their handlers.
/// bind_interrupts!(struct Irqs {
///    PIO0_IRQ_0 => InterruptHandler<PIO0>;
/// });
///
/// #[embassy_executor::main]
/// async fn main(spawner: Spawner) {
///     let rp_peripherals = embassy_rp::init(Default::default());
///     let (_net_device, mut _control, _runner) = utils::wifi_init!(rp_peripherals, Irqs);
///
///     loop {
///         cortex_m::asm::nop();
///     }
/// }
#[macro_export]
macro_rules! wifi_init {
    ($p:expr, $irq:expr) => {{
        // Wifi chip firmware.
        let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");

        // Initialize SPI for the WiFi chip.
        let pwr = embassy_rp::gpio::Output::new($p.PIN_23, embassy_rp::gpio::Level::Low);
        let cs = embassy_rp::gpio::Output::new($p.PIN_25, embassy_rp::gpio::Level::High);
        let mut pio = embassy_rp::pio::Pio::new($p.PIO0, $irq);
        let spi = cyw43_pio::PioSpi::new(
            &mut pio.common,
            pio.sm0,
            pio.irq0,
            cs,
            $p.PIN_24,
            $p.PIN_29,
            $p.DMA_CH0,
        );

        static STATE: static_cell::StaticCell<cyw43::State> = static_cell::StaticCell::new();
        let state = STATE.init(cyw43::State::new());

        // Initialize WiFi chip and set power management mode.
        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

        (net_device, control, runner)
    }};
}

/// Initializes the WiFi control handle and sets the power management mode.
#[macro_export]
macro_rules! wifi_ctrl_init {
    ($ctrl:expr,$pwr:expr) => {
        let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");
        $ctrl.init(clm).await;
        $ctrl.set_power_management($pwr).await;
    };
}
