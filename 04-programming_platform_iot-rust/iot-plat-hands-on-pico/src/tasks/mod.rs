//! Task management module for the IoT platform.

mod blink;
mod serial;
mod usb;

// Re-export tasks
pub use blink::blink_led_task;
pub use serial::send_messages_to_serial_task;
pub use usb::run_usb_device_task;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;

/// Shared LED state between blink and serial tasks
pub static LED_STATE: Signal<ThreadModeRawMutex, bool> = Signal::new();

