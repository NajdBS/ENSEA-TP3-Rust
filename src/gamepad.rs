
use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_stm32::Peri;

use crate::bsp_ensea::GamepadPins;

pub enum Button {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

pub struct GamepadState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

pub struct Gamepad {
    btn_top: Input<'static>,
    btn_bottom: Input<'static>,
    btn_left: Input<'static>,
    btn_right: Input<'static>,
    btn_center: Input<'static>,
}

impl Gamepad {
    pub fn new(
        top: Peri<'static, AnyPin>,
        bottom: Peri<'static, AnyPin>,
        left: Peri<'static, AnyPin>,
        right: Peri<'static, AnyPin>,
        center: Peri<'static, AnyPin>,
    ) -> Self {
        Self {
            btn_top: Input::new(top, Pull::Up),
            btn_bottom: Input::new(bottom, Pull::Up),
            btn_left: Input::new(left, Pull::Up),
            btn_right: Input::new(right, Pull::Up),
            btn_center: Input::new(center, Pull::Up),
        }
    }

    pub fn from_pins(pins: GamepadPins) -> Self {
        Self::new(pins.top, pins.bottom, pins.left, pins.right, pins.center)
    }

    pub fn is_pressed(&self, button: Button) -> bool {
        match button {
            Button::Top => self.btn_top.is_low(),
            Button::Bottom => self.btn_bottom.is_low(),
            Button::Left => self.btn_left.is_low(),
            Button::Right => self.btn_right.is_low(),
            Button::Center => self.btn_center.is_low(),
        }
    }

    pub fn poll(&self) -> GamepadState {
        GamepadState {
            top: self.btn_top.is_low(),
            bottom: self.btn_bottom.is_low(),
            left: self.btn_left.is_low(),
            right: self.btn_right.is_low(),
            center: self.btn_center.is_low(),
        }
    }
}