//! This is a platform agnostic Rust driver for the APDS9960 digital proximity, ambient light, RGB
//! and gesture sensor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the sensor. See: [`enable()`].
//! - Enable/disable delay between proximity and / or color / ambient light cycles. See: [`enable_wait()`].
//! - Enable/disable long delay between proximity and / or color / ambient light cycles. See: [`enable_wait_long()`].
//! - Proximity:
//!     - Enable/disable the proximity sensor. See: [`enable_proximity()`].
//!     - Enable/disable proximity interrupt generation. See: [`enable_proximity_interrupts()`].
//!     - Enable/disable proximity saturation interrupt generation. See: [`enable_proximity_saturation_interrupts()`].
//!     - Read the proximity data. See: [`read_proximity()`].
//!     - Check whether the proximity data is valid. See: [`is_proximity_data_valid()`].
//!     - Set the proximity interrupt low/high thresholds. See: [`set_proximity_low_threshold()`].
//!     - Set the proximity offsets. See [`set_proximity_offsets()`].
//! - Color / ambient light:
//!     - Enable/disable the color / ambient light sensor. See: [`enable_light()`].
//!     - Check whether the color / ambient light data is valid. See: [`is_light_data_valid()`].
//!     - Read the color / ambient light clear, red, green, blue channel data. See: [`read_light_red()`].
//!     - Set the color / ambient light integration time. See: [`set_light_integration_time()`].
//! - Gesture recognition:
//!     - Enable/disable gesture recognition. See: [`enable_gesture()`].
//!     - Enable/disable gesture mode. See: [`enable_gesture_mode()`].
//!     - Enable/disable gesture interrupts. See: [`enable_gesture_interrupts()`].
//!     - Read whether there is valid gesture data available. See: [`is_gesture_data_valid()`].
//!     - Read the amount of gesture data available. See: [`read_gesture_data_level()`].
//!     - Set the threshold of amount of available gesture data. See: [`set_gesture_data_level_threshold()`].
//!     - Read whether the gesture data has overflown. See: [`has_gesture_data_overflown()`].
//!     - Read the gesture data. See: [`read_gesture_data()`].
//!     - Set the gesture proximity entry/exit thresholds. See: [`set_gesture_proximity_entry_threshold()`].
//!     - Set the gesture offsets. See: [`set_gesture_offsets()`].
//! - Read the device ID. See: [`read_device_id()`].
//!
//! [`enable()`]: struct.Apds9960.html#method.enable
//! [`enable_wait()`]: struct.Apds9960.html#method.enable_wait
//! [`enable_wait_long()`]: struct.Apds9960.html#method.enable_wait_long
//!
//! [`enable_proximity()`]: struct.Apds9960.html#method.enable_proximity
//! [`enable_proximity_interrupts()`]: struct.Apds9960.html#method.enable_proximity_interrupts
//! [`enable_proximity_saturation_interrupts()`]: struct.Apds9960.html#method.enable_proximity_saturation_interrupts
//! [`read_proximity()`]: struct.Apds9960.html#method.read_proximity
//! [`is_proximity_data_valid()`]: struct.Apds9960.html#method.is_proximity_data_valid
//! [`set_proximity_low_threshold()`]: struct.Apds9960.html#method.set_proximity_low_threshold()
//! [`set_proximity_offsets()`]: struct.Apds9960.html#method.set_proximity_offsets
//!
//! [`enable_light()`]: struct.Apds9960.html#method.enable_light
//! [`is_light_data_valid()`]: struct.Apds9960.html#method.is_light_data_valid
//! [`read_light_red()`]: struct.Apds9960.html#method.read_light_red
//! [`set_light_integration_time()`]: struct.Apds9960.html#method.set_light_integration_time
//!
//! [`enable_gesture()`]: struct.Apds9960.html#method.enable_gesture
//! [`enable_gesture_mode()`]: struct.Apds9960.html#method.enable_gesture_mode
//! [`enable_gesture_interrupts()`]: struct.Apds9960.html#method.enable_gesture_interrupts
//! [`read_gesture_data_level()`]: struct.Apds9960.html#method.read_gesture_data_level
//! [`set_gesture_data_level_threshold()`]: struct.Apds9960.html#method.set_gesture_data_level_threshold
//! [`read_gesture_data()`]: struct.Apds9960.html#method.read_gesture_data
//! [`is_gesture_data_valid()`]: struct.Apds9960.html#method.is_gesture_data_valid
//! [`has_gesture_data_overflown()`]: struct.Apds9960.html#method.has_gesture_data_overflown
//! [`set_gesture_proximity_entry_threshold()`]: struct.Apds9960.html#method.set_gesture_proximity_entry_threshold
//! [`set_gesture_offsets()`]: struct.Apds9960.html#method.set_gesture_offsets
//! [`read_device_id()`]: struct.Apds9960.html#method.read_device_id
//!
//! ## The device
//!
//! The APDS-9960 device features advanced gesture detection, proximity detection, digital ambient
//! light sense (ALS) and color sense (RGBC).
//!
//! The communication is done through an I2C bidirectional bus.
//!
//! Datasheet:
//! - [APDS9960](https://docs.broadcom.com/docs/AV02-4191EN)
//!
//! ## Usage example
//! Please find additional examples in this repository: [apds9960-examples]
//!
//! [apds9960-examples]: https://github.com/eldruin/apds9960-examples

#![deny(missing_docs, unsafe_code, warnings)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;
extern crate nb;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Gesture FIFO data threshold.
///
/// This value is compared to the gesture data level to set data valid and generate an interruption.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GestureDataThreshold {
    /// Interrupt is generated and gesture data is set valid after 1 dataset is added to FIFO. (default)
    Th1,
    /// Interrupt is generated and gesture data is set valid after 4 datasets is added to FIFO.
    Th4,
    /// Interrupt is generated and gesture data is set valid after 8 datasets is added to FIFO.
    Th8,
    /// Interrupt is generated and gesture data is set valid after 16 datasets is added to FIFO.
    Th16,
}

const DEV_ADDR: u8 = 0x39;

struct Register;
impl Register {
    // const RAM_START  : u8 = 0x00;
    const ENABLE     : u8 = 0x80;
    const ATIME      : u8 = 0x81;
    // const WTIME      : u8 = 0x83;
    // const AILTL      : u8 = 0x84;
    // const AILTH      : u8 = 0x85;
    // const AIHTL      : u8 = 0x86;
    // const AIHTH      : u8 = 0x87;
    const PILT       : u8 = 0x89;
    const PIHT       : u8 = 0x8B;
    // const PERS       : u8 = 0x8C;
    const CONFIG1    : u8 = 0x8D;
    // const PPULSE     : u8 = 0x8E;
    // const CONTROL    : u8 = 0x8F;
    const CONFIG2    : u8 = 0x90;
    const ID         : u8 = 0x92;
    const STATUS     : u8 = 0x93;
    const CDATAL     : u8 = 0x94;
    const RDATAL     : u8 = 0x96;
    const GDATAL     : u8 = 0x98;
    const BDATAL     : u8 = 0x9A;
    const PDATA      : u8 = 0x9C;
    const POFFSET_UR : u8 = 0x9D;
    const POFFSET_DL : u8 = 0x9E;
    const GPENTH     : u8 = 0xA0;
    const GPEXTH     : u8 = 0xA1;
    const GCONFIG1   : u8 = 0xA2;
    const GOFFSET_U  : u8 = 0xA4;
    const GOFFSET_D  : u8 = 0xA5;
    const GOFFSET_L  : u8 = 0xA6;
    const GOFFSET_R  : u8 = 0xA7;
    const GCONFIG4   : u8 = 0xAB;
    const GFLVL      : u8 = 0xAE;
    const GSTATUS    : u8 = 0xAF;
    const GFIFO_U    : u8 = 0xFC;
}

trait BitFlags<T = Self> {
    const ADDRESS: u8;
    fn new(value: u8) -> T;
    fn with(&self, mask: u8, value: bool) -> T {
        if value {
            Self::new(self.value() | mask)
        } else {
            Self::new(self.value() & !mask)
        }
    }

    fn is(&self, mask: u8, value: bool) -> bool {
        ((self.value() & mask) != 0) == value
    }

    fn value(&self) -> u8;
}

mod register {
    use super::{BitFlags, Register};
    macro_rules! impl_bitflags {
        ($name:ident, $reg:ident) => {
            impl BitFlags for $name {
                const ADDRESS: u8 = Register::$reg;
                fn new(value: u8) -> Self {
                    Self { 0: value }
                }
                fn value(&self) -> u8 {
                    self.0
                }
            }
        };
    }

    #[derive(Debug, Default)]
    pub struct Enable(u8);
    impl Enable {
        pub const ALL: u8 = 0b1111_1111;
        pub const PON: u8 = 0b0000_0001;
        pub const AEN: u8 = 0b0000_0010;
        pub const PEN: u8 = 0b0000_0100;
        pub const WEN: u8 = 0b0000_1000;
        pub const PIEN: u8 = 0b0010_0000;
        pub const GEN: u8 = 0b0100_0000;
    }
    impl_bitflags!(Enable, ENABLE);

    #[derive(Debug)]
    pub struct Config1(u8);
    impl Config1 {
        pub const WLONG: u8 = 0b0000_0010;
    }
    impl_bitflags!(Config1, CONFIG1);

    impl Default for Config1 {
        fn default() -> Self {
            Self { 0: 0x40 }
        }
    }

    #[derive(Debug)]
    pub struct Config2(u8);
    impl Config2 {
        pub const PSIEN: u8 = 0b1000_0000;
    }
    impl_bitflags!(Config2, CONFIG2);

    impl Default for Config2 {
        fn default() -> Self {
            Self { 0: 1 }
        }
    }

    #[derive(Debug, Default)]
    pub struct GConfig1(u8);
    impl GConfig1 {
        pub const GFIFOTH1: u8 = 0b1000_0000;
        pub const GFIFOTH0: u8 = 0b0100_0000;
    }
    impl_bitflags!(GConfig1, GCONFIG1);

    #[derive(Debug, Default)]
    pub struct Status(u8);
    impl Status {
        pub const AVALID: u8 = 0b0000_0001;
        pub const PVALID: u8 = 0b0000_0010;
    }
    impl_bitflags!(Status, STATUS);

    #[derive(Debug, Default)]
    pub struct GConfig4(u8);
    impl GConfig4 {
        pub const GMODE: u8 = 0b0000_0001;
        pub const GIEN: u8 = 0b0000_0010;
    }
    impl_bitflags!(GConfig4, GCONFIG4);

    #[derive(Debug, Default)]
    pub struct GStatus(u8);
    impl GStatus {
        pub const GVALID: u8 = 0b0000_0001;
        pub const GFOV: u8 = 0b0000_0010;
    }
    impl_bitflags!(GStatus, GSTATUS);
}

/// APDS9960 device driver.
#[derive(Debug, Default)]
pub struct Apds9960<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    enable: register::Enable,
    config1: register::Config1,
    config2: register::Config2,
    gconfig1: register::GConfig1,
    gconfig4: register::GConfig4,
}

impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the APDS9960 device.
    pub fn new(i2c: I2C) -> Self {
        Apds9960 {
            i2c,
            enable: register::Enable::default(),
            config1: register::Config1::default(),
            config2: register::Config2::default(),
            gconfig1: register::GConfig1::default(),
            gconfig4: register::GConfig4::default(),
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

mod config;
mod gesture;
mod light;
mod proximity;
mod reading;
