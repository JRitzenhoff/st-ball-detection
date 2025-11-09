#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Pull, Speed};
use embassy_stm32::{exti::ExtiInput, gpio::Output};
use embassy_time::Instant;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut triggered_led = Output::new(p.PA5, Level::High, Speed::Low);
    triggered_led.set_low();

    // let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up); // user-button
    let mut button = ExtiInput::new(p.PD14, p.EXTI14, Pull::Up); // Arduino D2

    info!("Press the USER button...");

    const TRIGGER_THRESHOLD: u64 = 3000;

    let mut pre_trigger_time: Instant;
    let mut trigger_micros: u64;

    loop {
        // Capture the timestamp when the pulse starts
        button.wait_for_any_edge().await;
        pre_trigger_time = Instant::now();
        if button.is_low() {
            continue;
        }

        // Capture the timestamp when the pulse ends
        button.wait_for_any_edge().await;
        trigger_micros = pre_trigger_time.elapsed().as_micros();
        if button.is_high() {
            // This doesn't make any sense
            error!("Triggered interrupt and input is still high");
            continue;
        }

        if trigger_micros > TRIGGER_THRESHOLD {
            triggered_led.set_high();
            // info!("Captured {}", post_trigger_time);
        } else {
            triggered_led.set_low();
        }
    }
}
