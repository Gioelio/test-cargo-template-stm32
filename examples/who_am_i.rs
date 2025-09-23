//! Who_am_i example
//!
//! This example demonstrates how to read WHO_AM_I register using {{sensor}} driver
//! Default pins: PB8 (SCL), PB9 (SDA)

#![no_std]
#![no_main]

{% if framework == "stm32rs" -%}
use {defmt_rtt as _, panic_probe as _};
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    i2c::I2c,
    pac,
    prelude::*,
};
use defmt::*;

use lsm6dsv16x_rs::*;

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
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);

    // Wait a boot time
    let mut delay = cp.SYST.delay(&clocks);
    delay.delay_ms(5);

    let mut sensor = Lsm6dsv16x::new_i2c(i2c, I2CAddress::I2cAddH, delay);

    // Check device ID
    let id = sensor.device_id_get().unwrap();
    if id != ID {
        info!("Unexpected device ID: {}", id);
    } else {
        info!("Sensor found succesfully...");
    }

    loop {}
}
{% endif -%}

{% if framework == "embassy" -%}
use defmt::*;
use cortex_m::prelude::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    i2c::{self, I2c, Config as I2cConfig},
    time::khz,
    peripherals,
    dma::NoDma,
    bind_interrupts
};
use embassy_time::Delay;
use {defmt_rtt as _, panic_halt as _};
use lsm6dsv16x_rs::*;

#[defmt::panic_handler]
fn panic() -> ! {
    core::panic!("panic via `defmt::panic!`")
}

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("I2C example starting...");

    let p = embassy_stm32::init(Default::default());

    let i2c: I2c<_> = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        khz(100),
        I2cConfig::default(),
    );

    // Wait a boot time
    let mut delay = Delay;
    delay.delay_ms(5_u32);

    let mut sensor = Lsm6dsv16x::new_i2c(i2c, I2CAddress::I2cAddH, delay.clone());

    // Check device ID
    let id = sensor.device_id_get().unwrap();
    if id != ID {
        info!("Error: ID Sensor don't match expected!");
        loop {}
    } else {
        info!("Sensor found succesfully...");
    }

    loop {}
}

{% endif -%}
