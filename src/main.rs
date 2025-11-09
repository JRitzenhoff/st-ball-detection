#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Level, Output, Pull, Speed},
    peripherals,
    time::Hertz,
    timer::{self, pwm_input::PwmInput},
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

// Understanding the timers
//  In the stm32l4s5vi-overview.pdf file there is a chart with the mappings of timers to each input
//  On page 77/273, the row for PA7 shows the different timers connected

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
async fn pwm_read_timer(pwm_input: PwmInput<'static, peripherals::TIM17>) {
    // Some of the example logic is inspired by: https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/pwm_input.rs
    loop {
        Timer::after_millis(500).await;

        let period_ticks = pwm_input.get_period_ticks();
        let width_ticks = pwm_input.get_width_ticks();
        let duty_cycle = pwm_input.get_duty_cycle();

        info!(
            "period ticks: {} width ticks: {} duty cycle: {}",
            period_ticks, width_ticks, duty_cycle
        );

        if width_ticks == 0 {
            println!("Sensor timeout")
        } else if width_ticks > 1850 {
            // No detection.
            println!("No detection yet")
        } else {
            // Valid pulse width reading. Convert pulse width in microseconds to distance in millimeters.
            // let distance = (pulse_time - 1000) * 3 / 4;
            println!("Detected distance: {} mm", width_ticks);
        }
    }
}

// This asserts that the interrupt handler should be configured correctly
//  By `find . -iname _generated.rs` it's possible to see where the interrupts are actually defined
//  Unfortunately, TIM17 does not have a clean name like the others
bind_interrupts!(
    struct Irqs {
        TIM1_TRG_COM_TIM17 => timer::CaptureCompareInterruptHandler<peripherals::TIM17>;
    }
);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    // let mut led = Output::new(p.PB14, Level::High, Speed::Low);
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
    let mut pwm_input = PwmInput::new_ch1(p.TIM17, p.PB9, Pull::None, Hertz::khz(100));
    pwm_input.enable();
    // let _ = spawner.spawn(pwm_read_timer(pwm_input));

    loop {
        Timer::after_millis(500).await;

        let period_ticks = pwm_input.get_period_ticks();
        let width_ticks = pwm_input.get_width_ticks();
        let duty_cycle = pwm_input.get_duty_cycle();

        info!(
            "period ticks: {} width ticks: {} duty cycle: {}",
            period_ticks, width_ticks, duty_cycle
        );

        if width_ticks == 0 {
            println!("Sensor timeout")
        } else if width_ticks > 1850 {
            // No detection.
            println!("No detection yet")
        } else {
            // Valid pulse width reading. Convert pulse width in microseconds to distance in millimeters.
            // let distance = (pulse_time - 1000) * 3 / 4;
            println!("Detected distance: {} mm", width_ticks);
        }
    }

    // loop {
    //     led.set_high();
    //     Timer::after_millis(1000).await;
    //     led.set_low();
    //     Timer::after_millis(5000).await;
    // }
}
