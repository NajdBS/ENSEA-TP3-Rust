#![no_main]
#![no_std]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use tp3::bargraph::Bargraph;
use tp3::bsp_ensea::Board;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let mut bargraph = Bargraph::<8>::new(board.bargraph.into_array());
    bargraph.set_range(0, 100);

    info!("Bargraph example started");

    loop {
        for value in (0..=100).step_by(5) {
            info!("value = {}", value);
            bargraph.set_value(value);
            Timer::after(Duration::from_millis(100)).await;
        }

        for value in (0..=100).rev().step_by(5) {
            info!("value = {}", value);
            bargraph.set_value(value);
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}