use hal::blocking::i2c;
use {Apds9960, BitFlags, register::{Enable, GConfig4}, Error, DEV_ADDR};

macro_rules! impl_set_flag_reg {
    ($method:ident, $reg:ident) => {
        fn $method(&mut self, flag: u8, value: bool) -> Result<(), Error<E>> {
            let new = self.$reg.with(flag, value);
            self.config_register(&new)?;
            self.$reg = new;
            Ok(())
        }
    };
}

impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the APDS9960 device.
    pub fn new(i2c: I2C) -> Self {
        Apds9960 {
            i2c,
            enable: Enable::default(),
            gconfig4: GConfig4::default(),
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Turn power on.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PON, true)
    }

    /// Deactivate everything and put the device to sleep.
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::ALL, false)
    }

    /// Enable proximity detection
    pub fn enable_proximity(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PEN, true)
    }

    /// Disable proximity detection
    pub fn disable_proximity(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PEN, false)
    }

    /// Enable gesture detection
    pub fn enable_gesture(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::GEN, true)
    }

    /// Disable gesture detection
    pub fn disable_gesture(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::GEN, false)
    }

    /// Enable gesture mode.
    ///
    /// This can be automatically enabled (depending on proximity thresholds)
    /// and disabled (see GMODE on datasheet).
    pub fn enable_gesture_mode(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config4(GConfig4::GMODE, true)
    }

    /// Disable gesture mode.
    ///
    /// This can be automatically enabled (depending on proximity thresholds)
    /// and disabled (see GMODE on datasheet).
    pub fn disable_gesture_mode(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config4(GConfig4::GMODE, false)
    }
    }

    impl_set_flag_reg!(set_flag_enable, enable);
    impl_set_flag_reg!(set_flag_config4, gconfig4);


    fn config_register<T: BitFlags>(&mut self, reg: &T) -> Result<(), Error<E>> {
        self.write_register(T::ADDRESS, reg.value())
    }

    fn write_register(&mut self, address: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEV_ADDR, &[address, value])
            .map_err(Error::I2C)
    }
}
