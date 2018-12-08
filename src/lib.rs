//! This is a platform agnostic Rust driver for the APDS9960 digital proximity, ambient light, RGB
//! and gesture sensor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - TODO
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

#![deny(missing_docs, unsafe_code)]
//TODO #![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

const DEV_ADDR: u8 = 0x39;

struct Register;
impl Register {
    // const RAM_START  : u8 = 0x00;
    const ENABLE     : u8 = 0x80;
    // const ATIME      : u8 = 0x81;
    // const WTIME      : u8 = 0x83;
    // const AILTL      : u8 = 0x84;
    // const AILTH      : u8 = 0x85;
    // const AIHTL      : u8 = 0x86;
    // const AIHTH      : u8 = 0x87;
    // const PILT       : u8 = 0x89;
    // const PIHT       : u8 = 0x8B;
    // const PERS       : u8 = 0x8C;
    // const CONFIG1    : u8 = 0x8D;
    // const PPULSE     : u8 = 0x8E;
    // const CONTROL    : u8 = 0x8F;
    // const CONFIG2    : u8 = 0x90;
    const ID         : u8 = 0x92;
    // const STATUS     : u8 = 0x93;
    const PDATA      : u8 = 0x9C;
}

trait Configurable<T=Self> {
    const ADDRESS: u8;
    fn new(value: u8) -> T;
    fn with(&self, mask: u8, value: bool) -> T {
        if value {
            return Self::new(self.value() | mask);
        }
        else {
            return Self::new(self.value() & !mask);
        }
    }

    fn is(&self, mask: u8, value: bool) -> bool {
        ((self.value() & mask) != 0) == value
    }

    fn value(&self) -> u8;
}

mod register {
    use super::Configurable;

    #[derive(Debug, Default, Clone, Copy)]
    pub struct Enable(u8);
    impl Enable {
        pub const PON: u8 = 0b0000_0001;
        pub const PEN: u8 = 0b0000_0100;
    }
    impl Configurable for Enable {
        const ADDRESS: u8 = 0x80;
        fn new(value: u8) -> Self {
            Self {
                0: value
            }
        }
        fn value(&self) -> u8{
            self.0
        }
    }
}

/// APDS9960 device driver.
#[derive(Debug, Default)]
pub struct Apds9960<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    enable: register::Enable,
}

mod config;
mod reading;
