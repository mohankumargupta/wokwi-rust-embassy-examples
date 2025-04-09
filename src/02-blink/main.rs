#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Output, Level, OutputConfig};
use esp_hal::timer::systimer::SystemTimer;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.3.1
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);
    let mut led = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());  

    loop {
        Timer::after(Duration::from_secs(2)).await;
        led.set_high();
        Timer::after(Duration::from_secs(2)).await;
        led.set_low();
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}


