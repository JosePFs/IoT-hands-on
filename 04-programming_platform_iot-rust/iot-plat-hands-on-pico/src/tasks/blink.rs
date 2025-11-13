//! LED blinking task.

use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;

use super::LED_STATE;

/// Blinks the onboard LED and signals state changes.
///
/// This task toggles the LED every 500ms and updates the shared
/// LED_STATE signal for other tasks to consume.
#[embassy_executor::task]
pub async fn blink_led_task(mut led: Output<'static>) {
    loop {
        led.toggle();
        LED_STATE.signal(led.get_output_level() == Level::High);
        Timer::after_millis(500).await;
    }
}

