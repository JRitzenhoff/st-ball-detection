#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Pull;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::pwm_input::PwmInput;
// use embassy_stm32::{bind_interrupts, peripherals, timer};
// use embassy_time::Duration;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// NOTE: Found a message board on matrix: https://app.element.io/#/room/#embassy-rs:matrix.org/

/*
According to the stm32l45vi document:

TIM2, TIM3, TIM4, and TIM5 are general-purpose timers:
* They "feature four indepedent channels for input capture/output compare, PWM or one-pulse mode input."
* TIM2 and TIM5 have a 32-bit auto-reload up/downcounter and a 32-bit prescalar
* TIME4 and TIM4 have 16-bit auto-reload up/downcounter and 16-bit prescaler

TIM15, 16, and 16 are general-purpose timers with mid-range features:
* They have 16-bit auto-reload upcounters and 16-bit prescalers
* TIM15 has two channels and one complementary channel
* TIM16 and TIM17 have one channel and one complementary channel


According to the stm32l4-reference-manual document:
*/

// NOTE: This code is based heavily off of the stm32f4 pwm_input.rs example document
//  https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/pwm_input.rs

// NOTE: I'm not sure why the interrupt would be here. Seems to work perfectly fine without the compare handler
// bind_interrupts!(struct Irqs {
//     TIM2 => timer::CaptureCompareInterruptHandler<peripherals::TIM2>;
// });

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let sampling_rate = Hertz::khz(100);
    let micros_per_tick = (1_000_000.0 / sampling_rate.0 as f32) as u32;

    info!("Calculated {} micro-seconds per tick", micros_per_tick);

    let mut pwm_input = PwmInput::new_ch1(p.TIM3, p.PA6, Pull::None, sampling_rate);
    pwm_input.enable();

    loop {
        Timer::after_millis(500).await;
        let period = pwm_input.get_period_ticks();
        let width = pwm_input.get_width_ticks();
        let duty_cycle = pwm_input.get_duty_cycle();

        info!(
            "period ticks: {} width ticks: {} duty cycle: {}",
            period, width, duty_cycle
        );

        let width_micros = width * micros_per_tick;

        // Calculate taken from the Polulu Data-Sheet
        if width_micros < 1850 {
            let distance = 3.0 / 4.0 * (width_micros - 1000) as f32;
            info!("Detected {}mm", distance)
        } else {
            info!("No detection");
        }
    }
}
