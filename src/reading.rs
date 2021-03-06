use hal::blocking::i2c;
use {Apds9960, Error, Register, DEV_ADDR};

impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the device ID.
    ///
    /// This is per default `0xAB`.
    pub fn read_device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::ID)
    }

    pub(crate) fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.read_data(register, &mut data)?;
        Ok(data[0])
    }

    pub(crate) fn read_data(&mut self, register: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c
            .write_read(DEV_ADDR, &[register], data)
            .map_err(Error::I2C)
    }
}
