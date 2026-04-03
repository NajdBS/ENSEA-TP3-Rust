use embassy_stm32::gpio::AnyPin;
use embassy_stm32::Peri;

pub struct BargraphPins {
    /// LED0 – PC7
    pub led0: Peri<'static, AnyPin>,
    /// LED1 – PB2
    pub led1: Peri<'static, AnyPin>,
    /// LED2 – PA8
    pub led2: Peri<'static, AnyPin>,
    /// LED3 – PB1
    pub led3: Peri<'static, AnyPin>,
    /// LED4 – PB15
    pub led4: Peri<'static, AnyPin>,
    /// LED5 – PB4
    pub led5: Peri<'static, AnyPin>,
    /// LED6 – PB14
    pub led6: Peri<'static, AnyPin>,
    /// LED7 – PB5
    pub led7: Peri<'static, AnyPin>,
}

pub struct GpsPins {
    /// GPS_ENN – PB13
    pub enn: Peri<'static, AnyPin>,
}

pub struct GpioOutputPins {
    /// LD2 (green LED) – PA5
    pub ld2: Peri<'static, AnyPin>,
}

pub struct GpioInputPins {
    /// B1 (Blue PushButton) – PC13
    pub b1: Peri<'static, AnyPin>,
}

pub struct GamepadPins {
    /// BTN_TOP – PC8
    pub btn_top: Peri<'static, AnyPin>,
    /// BTN_BOTTOM – PB11
    pub btn_bottom: Peri<'static, AnyPin>,
    /// BTN_RIGHT – PC9
    pub btn_right: Peri<'static, AnyPin>,
    /// BTN_LEFT – PC6
    pub btn_left: Peri<'static, AnyPin>,
    /// BTN_CENTER – PC5
    pub btn_center: Peri<'static, AnyPin>,
}

pub struct MagnetometerPins {
    /// MAGNETO_STATUS – PC1
    pub status: Peri<'static, AnyPin>,
    /// MAGNETO_INT – PB0
    pub int: Peri<'static, AnyPin>,
}

pub struct RotaryEncoderPins {
    /// Encoder button (ENC_BTN) – PA15
    pub enc_btn: Peri<'static, AnyPin>,
    /// Encoder quadrature A – PA0
    pub quad_a: Peri<'static, AnyPin>,
    /// Encoder quadrature B – PA1
    pub quad_b: Peri<'static, AnyPin>,
}

pub struct StepperPins {
    /// Direction – PA7
    pub direction: Peri<'static, AnyPin>,
    /// Microstep MS1 – PA11
    pub ms1: Peri<'static, AnyPin>,
    /// Microstep MS2 – PB12
    pub ms2: Peri<'static, AnyPin>,
    /// Enable ENN – PA12
    pub enable: Peri<'static, AnyPin>,
    /// Step STP – PA6
    pub step: Peri<'static, AnyPin>,
}

pub struct Usart1Pins {
    /// USART1_TX – PA9
    pub tx: Peri<'static, AnyPin>,
    /// USART1_RX – PA10
    pub rx: Peri<'static, AnyPin>,
}

pub struct Usart2Pins {
    /// USART2_TX – PA2
    pub tx: Peri<'static, AnyPin>,
    /// USART2_RX – PB3
    pub rx: Peri<'static, AnyPin>,
}

pub struct Spi2Pins {
    /// SPI2_SCK – PB10
    pub sck: Peri<'static, AnyPin>,
    /// SPI2_MOSI – PC3
    pub mosi: Peri<'static, AnyPin>,
    /// SPI2_MISO – PC2
    pub miso: Peri<'static, AnyPin>,
    /// SPI_CS (chip select) – PC0
    pub cs: Peri<'static, AnyPin>,
}

pub struct I2c1Pins {
    /// I2C1_SCL – PB6
    pub scl: Peri<'static, AnyPin>,
    /// I2C1_SDA – PB7
    pub sda: Peri<'static, AnyPin>,
}

pub struct ConnectorPins {
    /// PC10 libre
    pub pc10: Peri<'static, AnyPin>,
    /// PC11 libre
    pub pc11: Peri<'static, AnyPin>,
    /// PC12 libre
    pub pc12: Peri<'static, AnyPin>,
    /// PB8 libre
    pub pb8: Peri<'static, AnyPin>,
    /// PB9 libre
    pub pb9: Peri<'static, AnyPin>,
    /// PD2 libre
    pub pd2: Peri<'static, AnyPin>,
}

/// Structure principale du Board (à décommenter progressivement)
pub struct Board {
    // pub bargraph_pins: BargraphPins,
    // pub gps_pins: GpsPins,
    // pub gpio_outputs: GpioOutputPins,
    // pub gpio_inputs: GpioInputPins,
    // pub gamepad_pins: GamepadPins,
    // pub magnetometer_pins: MagnetometerPins,
    // pub rotary_encoder_pins: RotaryEncoderPins,
    // pub stepper_pins: StepperPins,
    // pub usart1_pins: Usart1Pins,
    // pub usart2_pins: Usart2Pins,
    // pub spi2_pins: Spi2Pins,
    // pub i2c1_pins: I2c1Pins,
    // pub connector_pins: ConnectorPins,
}