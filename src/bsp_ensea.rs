#![allow(dead_code)]

use embassy_stm32::gpio::AnyPin;
use embassy_stm32::Peri;

/// Bargraph 8 LEDs
pub struct BargraphPins {
    pub led0: Peri<'static, AnyPin>, // PC7
    pub led1: Peri<'static, AnyPin>, // PB2
    pub led2: Peri<'static, AnyPin>, // PA8
    pub led3: Peri<'static, AnyPin>, // PB1
    pub led4: Peri<'static, AnyPin>, // PB15
    pub led5: Peri<'static, AnyPin>, // PB4
    pub led6: Peri<'static, AnyPin>, // PB14
    pub led7: Peri<'static, AnyPin>, // PB5
}

impl BargraphPins {
    pub fn into_array(self) -> [Peri<'static, AnyPin>; 8] {
        [
            self.led0,
            self.led1,
            self.led2,
            self.led3,
            self.led4,
            self.led5,
            self.led6,
            self.led7,
        ]
    }
}

pub struct Board {
    pub bargraph: BargraphPins,
}

impl Board {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            bargraph: BargraphPins {
                led0: p.PC7.into(),
                led1: p.PB2.into(),
                led2: p.PA8.into(),
                led3: p.PB1.into(),
                led4: p.PB15.into(),
                led5: p.PB4.into(),
                led6: p.PB14.into(),
                led7: p.PB5.into(),
            },
        }
    }
}