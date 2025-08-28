#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
};
use embassy_time::{Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

// NOTE: Can use a timer to read pulse width from the Polulu sensor:
//  https://blog.theembeddedrustacean.com/embassy-on-esp-timers

// Actual sensor datasheet:
//  https://www.pololu.com/product/5562

// Actual pinout datasheet (pg 22):
//  https://www.st.com/resource/en/user_manual/um2708-discovery-kit-for-iot-node-multichannel-communication-with-stm32l4-series-stmicroelectronics.pdf

#[embassy_executor::task]
async fn pwm_read_timer(mut pin: ExtiInput<'static>) {
    loop {
        // Wait for rising edge
        pin.wait_for_high().await;
        // Capture time instant at rising edge
        let inst = Instant::now();
        // Wait for falling edge
        pin.wait_for_low().await;
        // Calculate Duration
        let pwidth = Instant::checked_duration_since(&Instant::now(), inst).unwrap();
        // Print Duration
        println!("Sq Wave 1 Pulse Width is {}ms", pwidth.as_millis());
        // Uncomment below line to reduce console print frequency
        // Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led = Output::new(p.PB14, Level::High, Speed::Low);

    // According to the USER-manual -- pg 23
    let prox_trigger = ExtiInput::new(p.PD14, p.EXTI14, Pull::Down);

    // Actually spawn the read timer
    spawner.spawn(pwm_read_timer(prox_trigger)).unwrap();

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(100).await;

        info!("Hello world AGAIN!");
    }
}
