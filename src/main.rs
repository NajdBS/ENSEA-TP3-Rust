#![no_main]
#![no_std]

use core::sync::atomic::Ordering;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use tp3::bargraph::Bargraph;
use tp3::bsp_ensea::Board;
use tp3::encoder::Encoder;
use tp3::gamepad::Gamepad;
use tp3::shared::{
    load_direction, store_direction, BARGRAPH_LEVEL, EMERGENCY_STOP, ENCODER_POSITION,
    GAMEPAD_BOTTOM, GAMEPAD_CENTER, GAMEPAD_LEFT, GAMEPAD_RIGHT, GAMEPAD_TOP, STEPPER_SIGNAL,
    STEPPER_SPEED,
};
use tp3::stepper::{Direction, MicrosteppingMode, Stepper};

#[embassy_executor::task]
async fn bargraph_task(mut bargraph: Bargraph<8>) {
    loop {
        bargraph.wait_and_update().await;
    }
}

#[embassy_executor::task]
async fn gamepad_task(gamepad: Gamepad) {
    loop {
        let s = gamepad.poll();

        GAMEPAD_TOP.store(s.top, Ordering::Relaxed);
        GAMEPAD_BOTTOM.store(s.bottom, Ordering::Relaxed);
        GAMEPAD_LEFT.store(s.left, Ordering::Relaxed);
        GAMEPAD_RIGHT.store(s.right, Ordering::Relaxed);
        GAMEPAD_CENTER.store(s.center, Ordering::Relaxed);

        Timer::after(Duration::from_millis(50)).await;
    }
}

#[embassy_executor::task]
async fn encoder_task(encoder: Encoder) {
    let mut last_pos = encoder.position();

    loop {
        Timer::after(Duration::from_millis(200)).await;

        let pos = encoder.position();
        ENCODER_POSITION.store(pos, Ordering::Relaxed);

        let delta = pos - last_pos;
        last_pos = pos;

        let direction = if delta >= 0 {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        };

        let speed = (delta.unsigned_abs() * 100).clamp(0, 3000);

        if speed > 0 {
            EMERGENCY_STOP.store(false, Ordering::Relaxed);
        }

        store_direction(direction);
        STEPPER_SPEED.store(speed, Ordering::Relaxed);

        let level = (pos.unsigned_abs() % 101) as u32;
        Bargraph::<8>::update_value(level);

        STEPPER_SIGNAL.signal(());
    }
}

#[embassy_executor::task]
async fn stepper_update_task(mut stepper: Stepper) {
    stepper.set_microstepping(MicrosteppingMode::FullStep);

    loop {
        stepper.wait_for_update().await;
    }
}

#[embassy_executor::task]
async fn emergency_stop_task(button_pin: embassy_stm32::Peri<'static, AnyPin>) {
    let button = Input::new(button_pin, Pull::Up);
    let mut was_pressed = false;

    loop {
        let pressed = button.is_low();

        if pressed && !was_pressed {
            EMERGENCY_STOP.store(true, Ordering::Relaxed);
            STEPPER_SPEED.store(0, Ordering::Relaxed);
            STEPPER_SIGNAL.signal(());

            embassy_stm32::pac::TIM2.cnt().write_value(5_000);

            ENCODER_POSITION.store(0, Ordering::Relaxed);
            Bargraph::<8>::update_value(0);

            info!("EMERGENCY STOP");
        }

        was_pressed = pressed;
        Timer::after(Duration::from_millis(10)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    let board = Board::new(p);

    let mut bargraph = Bargraph::<8>::new(board.bargraph.into_array());
    bargraph.set_range(0, 100);

    let gamepad = Gamepad::from_pins(board.gamepad);
    let encoder = Encoder::new(board.encoder.tim2, board.encoder.pin_a, board.encoder.pin_b);
    let stepper = Stepper::from_pins(board.stepper);

    spawner.spawn(bargraph_task(bargraph).unwrap());
    spawner.spawn(gamepad_task(gamepad).unwrap());
    spawner.spawn(encoder_task(encoder).unwrap());
    spawner.spawn(stepper_update_task(stepper).unwrap());
    spawner.spawn(emergency_stop_task(board.encoder.btn).unwrap());

    loop {
        info!(
            "enc={} bar={} speed={} dir={:?} estop={} gp:[t={} b={} l={} r={} c={}]",
            ENCODER_POSITION.load(Ordering::Relaxed),
            BARGRAPH_LEVEL.load(Ordering::Relaxed),
            STEPPER_SPEED.load(Ordering::Relaxed),
            load_direction(),
            EMERGENCY_STOP.load(Ordering::Relaxed),
            GAMEPAD_TOP.load(Ordering::Relaxed),
            GAMEPAD_BOTTOM.load(Ordering::Relaxed),
            GAMEPAD_LEFT.load(Ordering::Relaxed),
            GAMEPAD_RIGHT.load(Ordering::Relaxed),
            GAMEPAD_CENTER.load(Ordering::Relaxed),
        );

        Timer::after(Duration::from_secs(1)).await;
    }
}