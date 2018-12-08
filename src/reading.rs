use hal::blocking::i2c;
use {Apds9960, DEV_ADDR, Error, Register};

impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the proximity sensor data
    pub fn read_proximity(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::PDATA)
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
