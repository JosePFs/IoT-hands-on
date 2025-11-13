#![no_std]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as UsbInterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::{Builder, Config};
use static_cell::StaticCell;

// Task management module
mod tasks;

/// Error type for the iot-plat crate.
#[derive(Debug)]
pub enum Error {
    SpawnError,
}

impl From<embassy_executor::SpawnError> for Error {
    fn from(_: embassy_executor::SpawnError) -> Self {
        Error::SpawnError
    }
}

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
});

/// Initializes and runs the IoT platform application.
///
/// This function sets up:
/// - LED blinking task on GPIO 25
/// - USB CDC-ACM serial communication
/// - USB device management
///
/// # Errors
///
/// Returns `Error::SpawnError` if any task fails to spawn.
pub async fn run(spawner: Spawner) -> Result<(), Error> {
    let peripherals = embassy_rp::init(Default::default());

    let led = Output::new(peripherals.PIN_25, Level::Low);

    let driver = Driver::new(peripherals.USB, Irqs);
    let config = Config::new(0x16c0, 0x27dd);

    static DEVICE_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 128]> = StaticCell::new();
    static STATE: StaticCell<State> = StaticCell::new();

    let mut builder = Builder::new(
        driver,
        config,
        DEVICE_DESC.init([0; 256]),
        CONFIG_DESC.init([0; 256]),
        BOS_DESC.init([0; 256]),
        CONTROL_BUF.init([0; 128]),
    );

    let cdc_acm = CdcAcmClass::new(&mut builder, STATE.init(State::new()), 64);
    let usb = builder.build();

    spawner.spawn(tasks::blink_led_task(led))?;
    spawner.spawn(tasks::run_usb_device_task(usb))?;
    spawner.spawn(tasks::send_messages_to_serial_task(cdc_acm))?;

    Ok(())
}
