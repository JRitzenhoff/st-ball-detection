#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::time::khz;
use embassy_stm32::timer::pwm_input::PwmInput;
use embassy_stm32::{Peri, bind_interrupts, peripherals, timer};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

/// Connect PB2 and PA6 with a 1k Ohm resistor

#[embassy_executor::task]
async fn blinky(led: Peri<'static, peripherals::PB2>) {
    let mut led = Output::new(led, Level::High, Speed::Low);

    let mut offset_counter: u64 = 0;
    const SLEEP_DELTA: u64 = 50;
    const DELTA_RESET: u64 = 10;

    const WIDTH_RESET: u8 = 5;
    let mut width_counter: u8 = 0;

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(300 + SLEEP_DELTA * offset_counter).await;

        info!("low");
        led.set_low();
        Timer::after_millis(300).await;

        width_counter = (width_counter + 1) % WIDTH_RESET;
        if width_counter == 0 {
            offset_counter = (offset_counter + 1) % DELTA_RESET;
        }
    }
}

bind_interrupts!(struct Irqs {
    TIM2 => timer::CaptureCompareInterruptHandler<peripherals::TIM2>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let _ = spawner.spawn(blinky(p.PB2));

    let mut pwm_input = PwmInput::new_ch1(p.TIM3, p.PA6, Pull::None, khz(10));
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
    }
}
