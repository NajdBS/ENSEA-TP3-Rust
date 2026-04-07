// src/shared.rs
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, AtomicU32, Ordering};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

use crate::stepper::Direction;

pub static BARGRAPH_LEVEL: AtomicU32 = AtomicU32::new(0);
pub static BARGRAPH_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub static ENCODER_POSITION: AtomicI32 = AtomicI32::new(0);

pub static STEPPER_SPEED: AtomicU32 = AtomicU32::new(0);
pub static STEPPER_DIRECTION_RAW: AtomicU8 = AtomicU8::new(0);
pub static STEPPER_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub static EMERGENCY_STOP: AtomicBool = AtomicBool::new(false);

pub static GAMEPAD_TOP: AtomicBool = AtomicBool::new(false);
pub static GAMEPAD_BOTTOM: AtomicBool = AtomicBool::new(false);
pub static GAMEPAD_LEFT: AtomicBool = AtomicBool::new(false);
pub static GAMEPAD_RIGHT: AtomicBool = AtomicBool::new(false);
pub static GAMEPAD_CENTER: AtomicBool = AtomicBool::new(false);

pub fn store_direction(dir: Direction) {
    let value = match dir {
        Direction::Clockwise => 0,
        Direction::CounterClockwise => 1,
    };
    STEPPER_DIRECTION_RAW.store(value, Ordering::Relaxed);
}

pub fn load_direction() -> Direction {
    match STEPPER_DIRECTION_RAW.load(Ordering::Relaxed) {
        0 => Direction::Clockwise,
        _ => Direction::CounterClockwise,
    }
}