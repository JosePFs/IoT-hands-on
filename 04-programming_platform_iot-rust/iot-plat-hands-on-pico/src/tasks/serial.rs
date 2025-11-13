//! USB serial communication task.

use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm::CdcAcmClass;

use super::LED_STATE;

/// Sends LED state messages over USB serial.
///
/// Waits for USB connection, then continuously reads LED state
/// and sends "LED: ON" or "LED: OFF" messages to the host.
#[embassy_executor::task]
pub async fn send_messages_to_serial_task(mut class: CdcAcmClass<'static, Driver<'static, USB>>) {
    loop {
        class.wait_connection().await;

        loop {
            let msg: &[u8] = match LED_STATE.wait().await {
                true => b"LED: ON\r\n",
                false => b"LED: OFF\r\n",
            };

            if class.write_packet(msg).await.is_err() {
                break; // USB disconnected, wait for reconnection
            }
        }
    }
}
