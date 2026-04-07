#![no_main]
#![no_std]

use defmt::*;
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

    loop {
        for v in (0..=100).step_by(10) {
            info!("bargraph value={}", v);
            bargraph.set_value(v);
            Timer::after(Duration::from_millis(250)).await;
        }

        for v in (0..=100).rev().step_by(10) {
            info!("bargraph value={}", v);
            bargraph.set_value(v);
            Timer::after(Duration::from_millis(250)).await;
        }
    }
}