#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use log::{error, info, warn};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);
    info!("Embassy initialized!\r");

    spawner.spawn(hello_world_task()).ok();

    loop {
        info!("\rHello world!\r");
        Timer::after(Duration::from_secs(5)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}

#[embassy_executor::task]
async fn hello_world_task() {
    let mut alternate = false;
    loop {
        if alternate {
            warn!("\rThis is a warning!\r");
        } else {
            error!("\r!This is an error. No sweat!\r");
        }
        alternate = !alternate;
        Timer::after(Duration::from_millis(2000)).await;
    }
}
