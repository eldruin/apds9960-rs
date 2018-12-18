use hal::blocking::i2c;
use {register::{Config1, Enable}, Apds9960, BitFlags, Error, Register, DEV_ADDR};

macro_rules! impl_set_flag_reg {
    ($method:ident, $reg:ident) => {
        pub(crate) fn $method(&mut self, flag: u8, value: bool) -> Result<(), Error<E>> {
            let new = self.$reg.with(flag, value);
            self.config_register(&new)?;
            self.$reg = new;
            Ok(())
        }
    };
}

/// Common configuration.
impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Turn power on.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PON, true)
    }

    /// Deactivate everything and put the device to sleep.
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::ALL, false)
    }

    /// Enable the wait feature.
    ///
    /// Enables delay between proximity and / or color and ambient light cycles.
    /// The duration of the wait can be configured with
    /// [`set_wait_time()`](struct.Apds9960.html#method.set_wait_time) and
    /// [`enable_wait_long()`](struct.Apds9960.html#method.enable_wait_long).
    pub fn enable_wait(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::WEN, true)
    }

    /// Disable the wait feature.
    pub fn disable_wait(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::WEN, false)
    }

    /// Enable long wait.
    ///
    /// The wait time will be multiplied by 12 so that each cycle takes 0.03s.
    /// See also: [`set_wait_time()`](struct.Apds9960.html#method.set_wait_time).
    ///
    /// Wait must be enabled with [`enable_wait()`](struct.Apds9960.html#method.enable_wait).
    pub fn enable_wait_long(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config1(Config1::WLONG, true)
    }

    /// Disable long wait.
    pub fn disable_wait_long(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config1(Config1::WLONG, false)
    }

    /// Set the waiting time between proximity and / or color and ambient light cycles.
    ///
    /// The value parameter must be a 2's complement of the number of cycles.
    ///
    /// Per default this is set to `0xFF` (1 cycle) and each cycle has a fixed duration of 2.78ms
    /// except if long wait is enabled, then this time is multiplied by 12.
    ///
    /// This must be set before enabling proximity and / or color and ambient light detection.
    ///
    /// Waiting must be enabled with [`enable_wait()`](struct.Apds9960.html#method.enable_wait).
    /// Long wait can be enabled with [`enable_wait_long()`](struct.Apds9960.html#method.enable_wait_long).
    pub fn set_wait_time(&mut self, value: u8) -> Result<(), Error<E>> {
        self.write_register(Register::WTIME, value)
    }

    impl_set_flag_reg!(set_flag_enable, enable);
    impl_set_flag_reg!(set_flag_config1, config1);
    impl_set_flag_reg!(set_flag_config2, config2);
    impl_set_flag_reg!(set_flag_gconfig4, gconfig4);

    pub(crate) fn config_register<T: BitFlags>(&mut self, reg: &T) -> Result<(), Error<E>> {
        self.write_register(T::ADDRESS, reg.value())
    }

    pub(crate) fn write_register(&mut self, address: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEV_ADDR, &[address, value])
            .map_err(Error::I2C)
    }
}
