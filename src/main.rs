#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Pull;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::pwm_input::PwmInput;
use embassy_stm32::{bind_interrupts, peripherals, timer};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    TIM2 => timer::CaptureCompareInterruptHandler<peripherals::TIM2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let sampling_rate = Hertz::khz(147);

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
    }
}
