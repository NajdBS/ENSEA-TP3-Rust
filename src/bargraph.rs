use core::cmp::min;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::Peri;

pub struct Bargraph<const N: usize> {
    leds: [Output<'static>; N],
    min: i32,
    max: i32,
}

impl<const N: usize> Bargraph<N> {
    pub fn new(pins: [Peri<'static, AnyPin>; N]) -> Self {
        // Level::High = LED OFF (active-low)
        let leds = pins.map(|pin| Output::new(pin, Level::High, Speed::Low));
        Self {
            leds,
            min: 0,
            max: N as i32,
        }
    }

    pub fn set_range(&mut self, min_value: i32, max_value: i32) {
        assert!(min_value < max_value);
        self.min = min_value;
        self.max = max_value;
    }

    pub fn clear(&mut self) {
        for led in &mut self.leds {
            led.set_high(); // active-low: HIGH = OFF
        }
    }

    pub fn fill(&mut self) {
        for led in &mut self.leds {
            led.set_low(); // active-low: LOW = ON
        }
    }

    pub fn set_led_count(&mut self, count: usize) {
        let count = min(count, N);
        for i in 0..N {
            if i < count {
                self.leds[i].set_low();  // active-low: ON
            } else {
                self.leds[i].set_high(); // active-low: OFF
            }
        }
    }

    pub fn set_value(&mut self, value: i32) {
        let count = self.value_to_led_count(value);
        self.set_led_count(count);
    }

    fn value_to_led_count(&self, value: i32) -> usize {
        if value <= self.min {
            return 0;
        }
        if value >= self.max {
            return N;
        }
        let span = (self.max - self.min) as i64;
        let offset = (value - self.min) as i64;
        ((offset * N as i64) / span) as usize
    }
}