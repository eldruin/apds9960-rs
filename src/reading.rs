use hal::blocking::i2c;
use {Apds9960, BitFlags, DEV_ADDR, Error, Register, register::Status};

impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the proximity sensor data.
    ///
    /// Returns `nb::Error::WouldBlock` as long as the data is not ready.
    pub fn read_proximity(&mut self) -> nb::Result<u8, Error<E>> {
        if !self.is_proximity_data_valid().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        self.read_register(Register::PDATA).map_err(nb::Error::Other)
    }

    /// Read whether the proximity sensor data is valid.
    ///
    /// This is checked internally in `read_proximity()` as well.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_proximity_data_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        Ok(Status::new(status).is(Status::PVALID, true))
    }

    /// Read the device ID.
    ///
    /// This is per default `0xAB`.
    pub fn read_device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::ID)
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEV_ADDR, &[register], &mut data)
            .map_err(Error::I2C)?;
        Ok(data[0])
    }
}
