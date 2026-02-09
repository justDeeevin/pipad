#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("droddyrox");

    #[cfg(debug_assertions)]
    spawner.spawn(keep_awake()).unwrap();

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        led.set_high();
        info!("high");
        Timer::after_millis(300).await;

        led.set_low();
        info!("low");
        Timer::after_millis(300).await;
    }
}

#[cfg(debug_assertions)]
#[embassy_executor::task]
// This is necessary to allow the debug probe to connect during program execution. Without this,
// embassy will put the CPU into sleep, preventing debug probe communication. I've already tried
// all manner of flag settings to allow debug probe comms while in sleep, but nothing worked.
async fn keep_awake() {
    loop {
        Timer::after_millis(1).await;
    }
}
