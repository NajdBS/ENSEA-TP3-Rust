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

/// Gamepad 5 boutons
pub struct GamepadPins {
    pub top: Peri<'static, AnyPin>,    // PC8
    pub bottom: Peri<'static, AnyPin>, // PB11
    pub right: Peri<'static, AnyPin>,  // PC9
    pub left: Peri<'static, AnyPin>,   // PC6
    pub center: Peri<'static, AnyPin>, // PC5
}

/// Encodeur rotatif
pub struct EncoderPins {
    pub tim2: Peri<'static, embassy_stm32::peripherals::TIM2>,
    pub pin_a: Peri<'static, embassy_stm32::peripherals::PA0>,
    pub pin_b: Peri<'static, embassy_stm32::peripherals::PA1>,
    pub btn: Peri<'static, AnyPin>, // PA15
}

/// Moteur pas à pas
pub struct StepperPins {
    pub dir: Peri<'static, AnyPin>,  // PA7
    pub ms1: Peri<'static, AnyPin>,  // PA11
    pub ms2: Peri<'static, AnyPin>,  // PB12
    pub enn: Peri<'static, AnyPin>,  // PA12
    pub step: Peri<'static, AnyPin>, // PA6
}

/// Bus I2C1 utilisé par l'écran OLED SSD1306
pub struct I2c1Pins {
    pub i2c: Peri<'static, embassy_stm32::peripherals::I2C1>,
    pub scl: Peri<'static, embassy_stm32::peripherals::PB6>,
    pub sda: Peri<'static, embassy_stm32::peripherals::PB7>,
}

pub struct Board {
    pub bargraph: BargraphPins,
    pub gamepad: GamepadPins,
    pub encoder: EncoderPins,
    pub stepper: StepperPins,
    pub i2c1: I2c1Pins,
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
            gamepad: GamepadPins {
                top: p.PC8.into(),
                bottom: p.PB11.into(),
                right: p.PC9.into(),
                left: p.PC6.into(),
                center: p.PC5.into(),
            },
            encoder: EncoderPins {
                tim2: p.TIM2,
                pin_a: p.PA0,
                pin_b: p.PA1,
                btn: p.PA15.into(),
            },
            stepper: StepperPins {
                dir: p.PA7.into(),
                ms1: p.PA11.into(),
                ms2: p.PB12.into(),
                enn: p.PA12.into(),
                step: p.PA6.into(),
            },
            i2c1: I2c1Pins {
                i2c: p.I2C1,
                scl: p.PB6,
                sda: p.PB7,
            },
        }
    }
}