#![no_std]
#![no_main]
use embassy_executor::Spawner;

// use esp_backtrace as _;
// use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*};
use embassy_stm32 as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // let peripherals = Peripherals::take();
    // let system = peripherals.SYSTEM.split();

    // let clocks = ClockControl::max(system.clock_control).freeze();
    // let delay = Delay::new(&clocks);

    // esp_println::logger::init_logger_from_env();

    loop {
        // log::info!("Hello world!");
        // delay.delay(500.millis());
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}

}