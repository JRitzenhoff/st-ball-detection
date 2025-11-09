#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    // pac::TIM17,
    peripherals::TIM17,
    time::Hertz,
    timer::pwm_input::PwmInput,
};
use embassy_time::{Duration, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

// NOTE: Can use a timer to read pulse width from the Polulu sensor:
//  https://blog.theembeddedrustacean.com/embassy-on-esp-timers

// Actual sensor datasheet:
//  https://www.pololu.com/product/5562

// Actual pinout datasheet (pg 22):
//  https://www.st.com/resource/en/user_manual/um2708-discovery-kit-for-iot-node-multichannel-communication-with-stm32l4-series-stmicroelectronics.pdf

// Examples for the stm32l4 board
//  https://github.com/embassy-rs/embassy/tree/main/examples/stm32l4

#[allow(dead_code)]
#[embassy_executor::task]
async fn exti_read_timer(mut pin: ExtiInput<'static>) {
    loop {
        // Wait for rising edge
        pin.wait_for_rising_edge().await;

        // Capture time instant at rising edge
        let inst = Instant::now();
        // Wait for falling edge
        pin.wait_for_falling_edge().await;
        // Calculate Duration
        let pwidth = Instant::checked_duration_since(&Instant::now(), inst).unwrap();
        // Print Duration
        println!("Sq Wave 1 Pulse Width is {}ms", pwidth.as_micros());
        // // Uncomment below line to reduce console print frequency
        // Timer::after(Duration::from_millis(1000)).await;
    }
}

#[allow(dead_code)]
#[embassy_executor::task]
async fn exti_mm_read(mut pin: ExtiInput<'static>) {
    // https://www.pololu.com/product/5472

    loop {
        // Wait for rising edge
        pin.wait_for_falling_edge().await;

        // Capture time instant at rising edge
        let inst = Instant::now();
        // Wait for falling edge
        pin.wait_for_rising_edge().await;
        // Calculate Duration
        let pwidth = Instant::checked_duration_since(&Instant::now(), inst).unwrap();
        // Print Duration

        let pulse_time = pwidth.as_micros();
        println!("Sq Wave 1 Pulse Width is {}ms", pulse_time);

        if pulse_time == 0 {
            println!("Sensor timeout")
        } else if pulse_time > 1850 {
            // No detection.
            println!("No detection yet")
        } else {
            // Valid pulse width reading. Convert pulse width in microseconds to distance in millimeters.
            let distance = (pulse_time - 1000) * 3 / 4;
            println!("Detected distance: {} mm", distance);
        }
    }
}

#[allow(dead_code)]
#[embassy_executor::task]
async fn pwm_read_timer(pwm_input: PwmInput<'static, TIM17>) {
    // Some of the example logic is inspired by: https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/pwm_input.rs

    let mut pwm_input = pwm_input;

    // Explictly enable the input
    pwm_input.enable();

    loop {
        // let pulse_time = pwm_input.get_period_ticks();
        let pulse_time = pwm_input.get_width_ticks();
        // let pulse_time = pwm_input.get_duty_cycle();

        if pulse_time == 0 {
            println!("Sensor timeout")
        } else if pulse_time > 1850 {
            // No detection.
            println!("No detection yet")
        } else {
            // Valid pulse width reading. Convert pulse width in microseconds to distance in millimeters.
            let distance = (pulse_time - 1000) * 3 / 4;
            println!("Detected distance: {} mm", distance);
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led = Output::new(p.PB14, Level::High, Speed::Low);
    // let mut second_led = Output::new(p.PA5, Level::High, Speed::Low);

    // // Use EXTI to detect the signal change
    let _button_trigger = ExtiInput::new(p.PC13, p.EXTI13, Pull::None);

    // Pull::Down -- Means this is triggered by 5V input
    // Pull::Up -- Means this is triggered by GND input
    // let prox_trigger = ExtiInput::new(p.PD14, p.EXTI14, Pull::Down);
    // Actually spawn the read timer
    // spawner.spawn(exti_read_timer(prox_trigger)).unwrap();
    // spawner.spawn(exti_mm_read(prox_trigger)).unwrap();

    // Use the PWM Input to detect the signal change
    let pwm_trigger = PwmInput::new(p.TIM17, p.PA7, Pull::Down, Hertz(142));
    spawner.spawn(pwm_read_timer(pwm_trigger)).unwrap();

    loop {
        led.set_high();
        Timer::after_millis(1000).await;
        led.set_low();
        Timer::after_millis(5000).await;
    }
}
