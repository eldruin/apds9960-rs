use hal::blocking::i2c;
use {
    register::Enable,
    Apds9960, Error,
};


impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    /// Enable color and ambient light detection
    pub fn enable_light(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::AEN, true)
    }

    /// Disable color and ambient light detection
    pub fn disable_light(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::AEN, false)
    }
}
