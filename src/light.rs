use hal::blocking::i2c;
use {
    register::{Enable, Status},
    Apds9960, BitFlags, Error, Register,
};

/// Color and ambient light.
impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    /// Enable color and ambient light detection.
    pub fn enable_light(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::AEN, true)
    }

    /// Disable color and ambient light detection.
    pub fn disable_light(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::AEN, false)
    }

    /// Set the color and ambient light integration time.
    ///
    /// The value parameter must be a 2's complement of the number of cycles.
    ///
    /// Per default this is set to `0xFF` (1 cycle) and each cycle has a fixed duration of 2.78ms.
    pub fn set_light_integration_time(&mut self, value: u8) -> Result<(), Error<E>> {
        self.write_register(Register::ATIME, value)
    }

    /// Read the color / ambient light sensor clear channel data.
    ///
    /// Returns `nb::Error::WouldBlock` as long as the data is not ready.
    pub fn read_light_clear(&mut self) -> nb::Result<u16, Error<E>> {
        self.read_light(Register::CDATAL)
    }

    /// Read the color / ambient light sensor red channel data.
    ///
    /// Returns `nb::Error::WouldBlock` as long as the data is not ready.
    pub fn read_light_red(&mut self) -> nb::Result<u16, Error<E>> {
        self.read_light(Register::RDATAL)
    }

    /// Read the color / ambient light sensor green channel data.
    ///
    /// Returns `nb::Error::WouldBlock` as long as the data is not ready.
    pub fn read_light_green(&mut self) -> nb::Result<u16, Error<E>> {
        self.read_light(Register::GDATAL)
    }

    /// Read the color / ambient light sensor blue channel data.
    ///
    /// Returns `nb::Error::WouldBlock` as long as the data is not ready.
    pub fn read_light_blue(&mut self) -> nb::Result<u16, Error<E>> {
        self.read_light(Register::BDATAL)
    }

    /// Read whether the color and ambient light sensor data is valid.
    ///
    /// This is checked internally in the `read_light_*()` methods as well.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_light_data_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        Ok(Status::new(status).is(Status::AVALID, true))
    }

    fn read_light(&mut self, register: u8) -> nb::Result<u16, Error<E>> {
        if !self.is_light_data_valid().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        let mut data = [0; 2];
        self.read_data(register, &mut data)
            .map_err(nb::Error::Other)?;
        Ok((u16::from(data[1]) << 8) | u16::from(data[0]))
    }
}
