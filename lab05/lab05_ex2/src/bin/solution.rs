#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;

use embassy_rp::gpio::{Input, Pull};

use embassy_rp::peripherals::{PIN_12, PIN_13};
use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Sender};

enum LedCommand {
    IncreaseI,
    DecreaseI,
}

static TOP: u16 = 0x8000;

static CHANNEL: Channel<ThreadModeRawMutex, LedCommand, 64> = Channel::new();

#[embassy_executor::task]
async fn button_a_pressed(
    mut button_a: Input<'static, PIN_12>,
    channel_sender: Sender<'static, ThreadModeRawMutex, LedCommand, 64>,
) {
    loop {
        button_a.wait_for_falling_edge().await;
        channel_sender.send(LedCommand::IncreaseI).await;
    }
}

#[embassy_executor::task]
async fn button_b_pressed(
    mut button_b: Input<'static, PIN_13>,
    channel_sender: Sender<'static, ThreadModeRawMutex, LedCommand, 64>,
) {
    loop {
        button_b.wait_for_falling_edge().await;
        channel_sender.send(LedCommand::DecreaseI).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let button_a = Input::new(peripherals.PIN_12, Pull::Up);
    let button_b = Input::new(peripherals.PIN_13, Pull::Up);

    let mut config: PwmConfig = Default::default();
    config.top = TOP;
    config.compare_a = config.top / 2;

    let mut pwm = Pwm::new_output_a(peripherals.PWM_CH0, peripherals.PIN_0, config.clone());

    spawner
        .spawn(button_a_pressed(button_a, CHANNEL.sender()))
        .unwrap();
    spawner
        .spawn(button_b_pressed(button_b, CHANNEL.sender()))
        .unwrap();

    loop {
        let value = CHANNEL.receive().await;
        match value {
            LedCommand::IncreaseI => config.compare_a += config.top / 10,
            LedCommand::DecreaseI => config.compare_a -= config.top / 10,
        }
        pwm.set_config(&config);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
