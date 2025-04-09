#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::timer::systimer::SystemTimer;
use log::info;

static SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);
  
    let led = Output::new(peripherals.GPIO4, Level::Low,  OutputConfig::default());  
    let button = Input::new(peripherals.GPIO5, InputConfig::default().with_pull(Pull::Up));
   
    info!("\rSpawning tasks...\r");
    spawner.spawn(button_task(button, &SIGNAL)).ok();
    spawner.spawn(led_task(led, &SIGNAL)).ok();
    info!("\rTasks spawned. Exiting main task gracefully.\r");

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}

#[embassy_executor::task]
async fn led_task(mut led: Output<'static>, signal: &'static Signal<CriticalSectionRawMutex, ()>) {
    loop {
      signal.wait().await;
      info!("\rTurn LED on for 2 seconds.!\r");
      led.set_high();
      Timer::after(Duration::from_millis(2000)).await;
      led.set_low();
    }
}

#[embassy_executor::task]
async fn button_task(mut button: Input<'static>, signal: &'static Signal<CriticalSectionRawMutex, ()>) {
   loop {
    button.wait_for_falling_edge().await;
    info!("\rButton pressed!\r");
    signal.signal(());
    Timer::after(Duration::from_millis(300)).await;  
   }
}

