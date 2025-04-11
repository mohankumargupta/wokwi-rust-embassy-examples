#![allow(dead_code)] // Allow unused code for the example

use core::marker::PhantomData;

use embassy_time::Duration;
use esp_hal::gpio::{Input, InputConfig, InputPin, Pull}; // Using embassy_hal for GPIO
use esp_hal::peripheral::Peripheral; // Import the Peripheral trait
use async_button::{Button, ButtonConfig, Mode}; // Import necessary items from async_button

// Define your CustomButton struct with a lifetime parameter 'd
pub struct CustomButton<'d, P>
where
    P: Peripheral + 'd, // Use 'd lifetime bound
    <P as Peripheral>::P: InputPin,
{
    // The inner Button instance from the async-button crate.
    // It takes an Input pin with lifetime 'd and uses Debounce for debouncing.
    inner: Button<Input<'d>>, // Use 'd lifetime for Input
    //_phantom: PhantomData<&'d ()>, // PhantomData to tie the lifetime 'd to the struct
   _phantom: PhantomData<P>, // PhantomData to tie the lifetime 'd to the struct
}

// Implementation block for CustomButton with lifetime 'd
impl<'d, P> CustomButton<'d, P>
where
    P: Peripheral + 'd, // Use 'd lifetime bound here too
    <P as Peripheral>::P: InputPin,
    {
    /// Creates a new CustomButton.
    ///
    /// # Arguments
    ///
    /// * `pin`: The embassy_hal Input pin connected to the button. Must live at least as long as 'd.
    /// * `pull`: The pull-up or pull-down configuration for the pin.
    /// * `debounce_duration`: The time duration to wait for the button state to stabilize.
    ///
    /// # Returns
    ///
    /// A new instance of `CustomButton`.
    pub fn new(pin: P, pull: Pull, debounce_duration: Duration) -> Self {
        // Create an Input pin managed by Embassy's HAL with lifetime 'd.
        let input_pin = Input::new(pin, InputConfig::default().with_pull(pull));

        const DOUBLE_CLICK: Duration = Duration::from_millis(350); // Example double-click duration
        const LONG_PRESS: Duration = Duration::from_millis(1000); // Example long-press duration     
        let mode = match pull {
            Pull::Up => Mode::PullUp,
            Pull::Down => Mode::PullDown,
            _ => Mode::PullUp
        };     
        let button_config = ButtonConfig::new(debounce_duration, DOUBLE_CLICK, LONG_PRESS, mode);

        // Create the inner async_button::Button with debouncing.
        let button_impl = Button::new(input_pin, button_config);

        Self { 
            inner: button_impl,
            _phantom: PhantomData, // Initialize the PhantomData to tie the lifetime 'd to the struct
        
        }
    }

    
}

// --- Example Usage (within an Embassy task) ---

// Assume you have initialized your peripherals (p) and executor.
// Replace `embassy_stm32::peripherals::PA0` with your actual pin type.
// #[embassy_executor::task]
// async fn button_handler(pin: embassy_stm32::peripherals::PA0) {
//     // Create the button instance with a pull-up resistor and 50ms debounce time.
//     // Adjust Pull::Up/Pull::Down based on your hardware circuit.
//     let mut button = CustomButton::new(pin, Pull::Up, Duration::from_millis(50));
//     defmt::info!("Button initialized. Waiting for press..."); // Use your logging framework
//
//     loop {
//         // Wait for the button to be pressed
//         button.wait_for_press().await;
//         defmt::info!("Button PRESSED!");
//
//         // Optional: Wait for the button to be released
//         button.wait_for_release().await;
//         defmt::info!("Button RELEASED!");
//
//         // Example: Wait for a specific edge (Falling edge typically means press for Pull::Up)
//         // button.wait_for_edge(Edge::Falling).await;
//         // defmt::info!("Button Falling Edge Detected (Pressed)!");
//         // button.wait_for_edge(Edge::Rising).await;
//         // defmt::info!("Button Rising Edge Detected (Released)!");
//
//         // Add a small delay to prevent busy-looping if needed,
//         // though wait_for_* methods are blocking asynchronously.
//         // Timer::after(Duration::from_millis(10)).await;
//     }
// }

/*
// --- How to potentially run this in your main ---
// #[embassy_executor::main]
// async fn main(spawner: Spawner) {
//     let p = embassy_stm32::init(Default::default()); // Or your specific HAL init
//     defmt::info!("Peripherals initialized.");
//
//     // Spawn the button handler task, passing the specific pin
//     spawner.spawn(button_handler(p.PA0)).unwrap(); // Use the correct pin
//
//     // Keep the main task alive or do other work
//     loop {
//         Timer::after(Duration::from_secs(1)).await;
//         // Optionally print status or yield
//         // defmt::info!("Main loop tick.");
//     }
// }
*/