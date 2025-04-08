//! Template project for Rust on ESP32 (`no_std`) based on [`esp-hal`](https://github.com/esp-rs/esp-hal)
//!
//! Useful resources:
//! - [The Rust on ESP Book](https://docs.esp-rs.org/book/)
//! - [Embedded Rust (no_std) on Espressif](https://docs.esp-rs.org/no_std-training/)
//! - [Matrix channel](https://matrix.to/#/#esp-rs:matrix.org)

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::timer::timg::TimerGroup;
use log::{error, info, warn};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    esp_println::logger::init_logger(log::LevelFilter::Info);
    // let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    // let _peripherals = esp_hal::init(config);

    //let delay = Delay::new();

    //esp_println::logger::init_logger_from_env();
    info!("About to spawn hello world task!\r");
    spawner.spawn(hello_world()).ok();
    info!("Just spawned hello world task!\r");
    loop {
        Timer::after(Duration::from_secs(5)).await;
    }
}

#[embassy_executor::task]
async fn hello_world() {
    loop {
        warn!("This is just a warning!\r");
        error!("This is just an error. DANGER DANGER!\r");
        esp_println::println!("Garden variety println!\r");
        Timer::after(Duration::from_secs(5)).await;
    }
}
