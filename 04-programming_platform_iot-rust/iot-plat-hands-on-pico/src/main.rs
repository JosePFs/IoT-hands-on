#![no_std]
#![no_main]

use core::future::pending;

use embassy_executor::Spawner;

use panic_halt as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    match iot_hands_on_pico::run(spawner).await {
        Ok(()) => {}
        Err(e) => panic!("Application error: {:?}", e),
    }

    pending::<()>().await;
}
