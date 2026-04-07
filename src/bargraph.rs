// src/bargraph.rs
use core::sync::atomic::Ordering;

use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::Peri;

use crate::shared::{BARGRAPH_LEVEL, BARGRAPH_SIGNAL};

pub struct Bargraph<const N: usize> {
    leds: [Output<'static>; N],
    min: u32,
    max: u32,
}

impl<const N: usize> Bargraph<N> {
    pub fn new(pins: [Peri<'static, AnyPin>; N]) -> Self {
        let leds = pins.map(|pin| Output::new(pin, Level::Low, Speed::Low));

        Self {
            leds,
            min: 0,
            max: 100,
        }
    }

    pub fn set_range(&mut self, min: u32, max: u32) {
        self.min = min;
        self.max = max.max(min + 1);
    }

    pub fn set_value(&mut self, value: u32) {
        let value = value.clamp(self.min, self.max);
        let span = self.max - self.min;
        let rel = value - self.min;

        let lit = if rel == 0 {
            0
        } else {
            (((rel as usize) * N) + (span as usize) - 1) / (span as usize)
        }
        .min(N);

        for (index, led) in self.leds.iter_mut().enumerate() {
            if index < lit {
                led.set_high();
            } else {
                led.set_low();
            }
        }
    }

    pub fn clear(&mut self) {
        for led in self.leds.iter_mut() {
            led.set_low();
        }
    }

    pub fn update_value(new_value: u32) {
        BARGRAPH_LEVEL.store(new_value, Ordering::Relaxed);
        BARGRAPH_SIGNAL.signal(());
    }

    pub async fn wait_and_update(&mut self) {
        BARGRAPH_SIGNAL.wait().await;
        let value = BARGRAPH_LEVEL.load(Ordering::Relaxed);
        self.set_value(value);
    }
}