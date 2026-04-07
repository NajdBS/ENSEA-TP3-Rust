use core::fmt::Write;

use embassy_stm32::i2c::{Config as I2cConfig, I2c, Master};
use embassy_stm32::mode::Blocking;
use embassy_stm32::time::Hertz;
use embassy_stm32::Peri;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};

use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig},
    prelude::{DisplayRotation, DisplaySize128x64, I2CInterface},
    I2CDisplayInterface, Ssd1306,
};

use crate::stepper::Direction;

pub struct OledDisplay {
    display: Ssd1306<
        I2CInterface<I2c<'static, Blocking, Master>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl OledDisplay {
    pub fn new(
        i2c: Peri<'static, embassy_stm32::peripherals::I2C1>,
        scl: Peri<'static, embassy_stm32::peripherals::PB6>,
        sda: Peri<'static, embassy_stm32::peripherals::PB7>,
    ) -> Self {
        let mut cfg = I2cConfig::default();
        cfg.frequency = Hertz(400_000);

        let i2c = I2c::new_blocking(i2c, scl, sda, cfg);

        let interface = I2CDisplayInterface::new(i2c);

        let mut display = Ssd1306::new(
            interface,
            DisplaySize128x64,
            DisplayRotation::Rotate0,
        )
        .into_buffered_graphics_mode();

        display.init().ok();
        display.flush().ok();

        Self { display }
    }

    pub fn clear(&mut self) {
        self.display.clear(BinaryColor::Off).ok();
        self.display.flush().ok();
    }

    pub fn show_status(
        &mut self,
        speed: u32,
        direction: Direction,
        encoder_pos: i32,
        gp_top: bool,
        gp_bottom: bool,
        gp_left: bool,
        gp_right: bool,
        gp_center: bool,
    ) {
        self.display.clear(BinaryColor::Off).ok();

        let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

        let dir_str = match direction {
            Direction::Clockwise => "CW",
            Direction::CounterClockwise => "CCW",
        };

        let mut line1 = heapless::String::<32>::new();
        let mut line2 = heapless::String::<32>::new();
        let mut line3 = heapless::String::<32>::new();
        let mut line4 = heapless::String::<32>::new();

        let _ = write!(line1, "Speed: {}", speed);
        let _ = write!(line2, "Dir: {}", dir_str);
        let _ = write!(line3, "Enc: {}", encoder_pos);
        let _ = write!(
            line4,
            "T{} B{} L{} R{} C{}",
            gp_top as u8,
            gp_bottom as u8,
            gp_left as u8,
            gp_right as u8,
            gp_center as u8
        );

        Text::new(&line1, Point::new(0, 10), text_style)
            .draw(&mut self.display)
            .ok();
        Text::new(&line2, Point::new(0, 24), text_style)
            .draw(&mut self.display)
            .ok();
        Text::new(&line3, Point::new(0, 38), text_style)
            .draw(&mut self.display)
            .ok();
        Text::new(&line4, Point::new(0, 52), text_style)
            .draw(&mut self.display)
            .ok();

        Rectangle::new(Point::new(96, 0), Size::new(32, 64))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut self.display)
            .ok();

        self.display.flush().ok();
    }
}