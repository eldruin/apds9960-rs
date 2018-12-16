use hal::blocking::i2c;
use {
    register::{Enable, Status},
    Apds9960, BitFlags, Error, Register,
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

    /// Read whether the color and ambient light sensor data is valid.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_light_data_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        Ok(Status::new(status).is(Status::AVALID, true))
    }
}
