#![no_main]
#![no_std]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use tp3::bsp_ensea::Board;
use tp3::gamepad::Gamepad;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let gamepad = Gamepad::from_pins(board.gamepad);

    loop {
        let s = gamepad.poll();
        info!(
            "top={} bottom={} left={} right={} center={}",
            s.top, s.bottom, s.left, s.right, s.center
        );
        Timer::after(Duration::from_millis(200)).await;
    }
}