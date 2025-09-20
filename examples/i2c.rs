//! I2C example
//!
//! This example demonstrates I2C communication
//! Default pins: PB8 (SCL), PB9 (SDA)

#![no_std]
#![no_main]

{% if framework == "stm32rs" -%}
use {defmt_rtt as _, panic_probe as _};
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    gpio::{Output, PushPull},
    pac,
    prelude::*,
};
use defmt::*;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(48.MHz()).freeze();

    let gpiob = dp.GPIOB.split();

    // Configure I2C pins
    let scl = gpiob.pb8.into_alternate().set_open_drain();
    let sda = gpiob.pb9.into_alternate().set_open_drain();

    // Create I2C interface
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);

    // Wait a boot time
    let mut delay = cp.SYST.delay(&clocks);
    delay.delay_ms(5);

    // Example: Scan for I2C devices
    for addr in 0x08..0x78 {
        if i2c.write(addr, &[]).is_ok() {
            // Device found at address
            // In a real application, you would handle this
            info!("Device found at addr: {addr}");
        }
    }

    loop {}
}
{% endif -%}

{% if framework == "embassy" -%}
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    i2c::{self, I2c},
    time::Hertz,
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_halt as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("I2C example starting...");

    let p = embassy_stm32::init(Default::default());

    let mut config = i2c::Config::default();
    config.frequency = Hertz(100_000); // 400 kHz

    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8, // SCL
        p.PB9, // SDA
        config,
    );

    loop {
        info!("Scanning I2C bus...");

        // Example: Scan for I2C devices
        for addr in 0x08..0x78_u8 {
            match i2c.write(addr, &[]).await {
                Ok(_) => info!("Device found at address 0x{:02X}", addr),
                Err(_) => {}, // No device at this address
            }
        }

        Timer::after_secs(2).await;
    }
}
{% endif -%}
