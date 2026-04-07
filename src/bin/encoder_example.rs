#![no_main]
#![no_std]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use tp3::bsp_ensea::Board;
use tp3::encoder::Encoder;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let encoder = Encoder::new(
        board.encoder.tim2,
        board.encoder.pin_a,
        board.encoder.pin_b,
    );

    loop {
        info!("encoder position={}", encoder.position());
        Timer::after(Duration::from_millis(200)).await;
    }
}