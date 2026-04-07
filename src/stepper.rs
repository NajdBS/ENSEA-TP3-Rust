use core::sync::atomic::Ordering;

use defmt::Format;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::Peri;

use crate::shared::{load_direction, EMERGENCY_STOP, STEPPER_SIGNAL, STEPPER_SPEED};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Format)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Format)]
pub enum MicrosteppingMode {
    FullStep,
    HalfStep,
    QuarterStep,
    EighthStep,
}

pub struct Stepper {
    dir: Output<'static>,
    ms1: Output<'static>,
    ms2: Output<'static>,
    enn: Output<'static>,
    step: Output<'static>,
    speed_hz: u32,
    direction: Direction,
}

impl Stepper {
    pub fn new(
        dir: Peri<'static, AnyPin>,
        ms1: Peri<'static, AnyPin>,
        ms2: Peri<'static, AnyPin>,
        enn: Peri<'static, AnyPin>,
        step: Peri<'static, AnyPin>,
    ) -> Self {
        let mut s = Self {
            dir: Output::new(dir, Level::Low, Speed::Medium),
            ms1: Output::new(ms1, Level::Low, Speed::Medium),
            ms2: Output::new(ms2, Level::Low, Speed::Medium),
            enn: Output::new(enn, Level::High, Speed::Medium),
            step: Output::new(step, Level::Low, Speed::Medium),
            speed_hz: 0,
            direction: Direction::Clockwise,
        };

        s.set_microstepping(MicrosteppingMode::FullStep);
        s.disable();
        s
    }

    pub fn from_pins(pins: crate::bsp_ensea::StepperPins) -> Self {
        Self::new(pins.dir, pins.ms1, pins.ms2, pins.enn, pins.step)
    }

    pub fn set_speed(&mut self, speed_hz: u32, direction: Direction) {
        self.speed_hz = speed_hz;
        self.direction = direction;

        match direction {
            Direction::Clockwise => self.dir.set_high(),
            Direction::CounterClockwise => self.dir.set_low(),
        }
    }

    pub fn enable(&mut self) {
        self.enn.set_low();
    }

    pub fn disable(&mut self) {
        self.enn.set_high();
    }

    pub fn set_microstepping(&mut self, mode: MicrosteppingMode) {
        match mode {
            MicrosteppingMode::FullStep => {
                self.ms1.set_low();
                self.ms2.set_low();
            }
            MicrosteppingMode::HalfStep => {
                self.ms1.set_high();
                self.ms2.set_low();
            }
            MicrosteppingMode::QuarterStep => {
                self.ms1.set_low();
                self.ms2.set_high();
            }
            MicrosteppingMode::EighthStep => {
                self.ms1.set_high();
                self.ms2.set_high();
            }
        }
    }

    pub fn pulse(&mut self) {
        self.step.set_high();
        cortex_m::asm::delay(800);
        self.step.set_low();
    }

    pub async fn wait_for_update(&mut self) {
        STEPPER_SIGNAL.wait().await;

        let speed = STEPPER_SPEED.load(Ordering::Relaxed);
        let direction = load_direction();

        if EMERGENCY_STOP.load(Ordering::Relaxed) || speed == 0 {
            self.disable();
            return;
        }

        self.set_speed(speed, direction);
        self.enable();
        self.pulse();
    }
}