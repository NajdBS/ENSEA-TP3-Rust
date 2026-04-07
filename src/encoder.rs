
use embassy_stm32::timer::qei::{Config, Qei};
use embassy_stm32::Peri;

pub const ARR: u32 = 10_000;
pub const CENTER: u32 = ARR / 2;

pub struct Encoder {
    qei: Qei<'static, embassy_stm32::peripherals::TIM2>,
}

impl Encoder {
    pub fn new(
        tim2: Peri<'static, embassy_stm32::peripherals::TIM2>,
        pin_a: Peri<'static, embassy_stm32::peripherals::PA0>,
        pin_b: Peri<'static, embassy_stm32::peripherals::PA1>,
    ) -> Self {
        let qei = Qei::new(tim2, pin_a, pin_b, Config::default());

        let tim2_pac = embassy_stm32::pac::TIM2;
        tim2_pac.arr().write_value(ARR);
        tim2_pac.cnt().write_value(CENTER);

        Self { qei }
    }

    pub fn position(&self) -> i32 {
        self.qei.count() as i32 - CENTER as i32
    }

    pub fn set_position(&self, position: i32) {
        let raw = (CENTER as i32 + position).clamp(0, ARR as i32) as u32;
        embassy_stm32::pac::TIM2.cnt().write_value(raw);
    }

    pub fn reset(&self) {
        embassy_stm32::pac::TIM2.cnt().write_value(CENTER);
    }
}