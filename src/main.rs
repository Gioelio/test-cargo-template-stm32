#![no_std]
#![no_main]

{% if framework == "stm32rs" -%}
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    gpio::{Output, PushPull},
    pac,
    prelude::*,
    timer::Timer,
};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Acquire the GPIO peripheral
    let gpioc = dp.GPIOC.split();

    // Configure PC13 as a push-pull output (onboard LED on many STM32 boards)
    let mut led = gpioc.pc13.into_push_pull_output();

    // Create a delay abstraction based on SysTick
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    loop {
        // Toggle the LED
        led.toggle();

        // Wait for the timer to expire
        nb::block!(timer.wait()).unwrap();
    }
}
{% endif -%}

{% if framework == "embassy" -%}
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_halt as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    // Configure PC13 as output (onboard LED on many STM32 boards)
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(300).await;

        info!("low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}
{% endif -%}
