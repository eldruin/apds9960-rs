use hal::blocking::i2c;
use {register::Enable, Apds9960, BitFlags, Error, DEV_ADDR};

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
    /// The duration of the wait can be configured with `set_wait_time()` and `enable_wait_long()`.
    pub fn enable_wait(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::WEN, true)
    }

    /// Disable the wait feature.
    pub fn disable_wait(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::WEN, false)
    }
    impl_set_flag_reg!(set_flag_enable, enable);
    impl_set_flag_reg!(set_flag_gconfig4, gconfig4);
    impl_set_flag_reg!(set_flag_config2, config2);

    pub(crate) fn config_register<T: BitFlags>(&mut self, reg: &T) -> Result<(), Error<E>> {
        self.write_register(T::ADDRESS, reg.value())
    }

    pub(crate) fn write_register(&mut self, address: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEV_ADDR, &[address, value])
            .map_err(Error::I2C)
    }
}
