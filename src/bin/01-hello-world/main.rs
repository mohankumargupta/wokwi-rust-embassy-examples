//! Template project for Rust on ESP32 (`no_std`) based on [`esp-hal`](https://github.com/esp-rs/esp-hal)
//!
//! Useful resources:
//! - [The Rust on ESP Book](https://docs.esp-rs.org/book/)
//! - [Embedded Rust (no_std) on Espressif](https://docs.esp-rs.org/no_std-training/)
//! - [Matrix channel](https://matrix.to/#/#esp-rs:matrix.org)

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::CpuClock, delay::Delay, main};
use log::{info, error};

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let delay = Delay::new();

    //esp_println::logger::init_logger_from_env();
    esp_println::logger::init_logger(log::LevelFilter::Info);
    info!("Hello world mohan!\r");
    error!("Danger\r");
    esp_println::println!("Hello world mohan println!\r");
    loop {
        delay.delay_millis(2000);
    }
}
