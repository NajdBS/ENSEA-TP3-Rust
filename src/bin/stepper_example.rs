#![no_main]
#![no_std]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use tp3::bsp_ensea::Board;
use tp3::stepper::{Direction, MicrosteppingMode, Stepper};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let mut stepper = Stepper::from_pins(board.stepper);
    stepper.set_microstepping(MicrosteppingMode::FullStep);

    loop {
        info!("stepper CW");
        stepper.set_speed(200, Direction::Clockwise);
        stepper.enable();
        for _ in 0..200 {
            stepper.pulse().await;
            Timer::after(Duration::from_millis(5)).await;
        }
        stepper.disable();

        Timer::after(Duration::from_millis(500)).await;

        info!("stepper CCW");
        stepper.set_speed(200, Direction::CounterClockwise);
        stepper.enable();
        for _ in 0..200 {
            stepper.pulse().await;
            Timer::after(Duration::from_millis(5)).await;
        }
        stepper.disable();

        Timer::after(Duration::from_millis(500)).await;
    }
}