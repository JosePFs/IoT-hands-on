//! USB device management task.

use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::UsbDevice;

/// Runs the USB device, keeping it alive and responsive.
///
/// This task must run continuously to maintain USB enumeration
/// and respond to host requests.
#[embassy_executor::task]
pub async fn run_usb_device_task(mut usb_device: UsbDevice<'static, Driver<'static, USB>>) -> ! {
    usb_device.run().await
}

