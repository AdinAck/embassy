#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

#[path = "../example_common.rs"]
mod example_common;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::Peripherals;
use embedded_hal::digital::v2::OutputPin;
use example_common::*;
use embassy_stm32::rng::Random;
use embassy::traits::rng::Random as _;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    let mut led = Output::new(p.PB14, Level::High, Speed::Low);

    let mut rng = Random::new(p.RNG);

    loop {
        info!("high {}", unwrap!(rng.next(16).await) );
        unwrap!(led.set_high());
        Timer::after(Duration::from_millis(500)).await;

        info!("low {}", unwrap!(rng.next(16).await) );
        unwrap!(led.set_low());
        Timer::after(Duration::from_millis(500)).await;
    }
}
