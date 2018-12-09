use hal::blocking::i2c;
use {Apds9960, BitFlags, register, Error, Register, DEV_ADDR};


impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the APDS9960 device.
    pub fn new(i2c: I2C) -> Self {
        Apds9960 {
            i2c,
            enable: register::Enable::default(),
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Turn power on.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let new = self.enable.with(register::Enable::PON, true);
        self.config_register(new)?;
        self.enable = new;
        Ok(())
    }

    /// Deactivate everything and put the device to sleep.
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::ENABLE, 0)
    }

    /// Enable proximity detection
    pub fn enable_proximity(&mut self) -> Result<(), Error<E>> {
        let new = self.enable.with(register::Enable::PEN, true);
        self.config_register(new)?;
        self.enable = new;
        Ok(())
    }

    /// Disable proximity detection
    pub fn disable_proximity(&mut self) -> Result<(), Error<E>> {
        let new = self.enable.with(register::Enable::PEN, false);
        self.config_register(new)?;
        self.enable = new;
        Ok(())
    }

    fn config_register<T: BitFlags>(&mut self, reg: T) -> Result<(), Error<E>> {
        self.write_register(T::ADDRESS, reg.value())
    }

    fn write_register(&mut self, address: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEV_ADDR, &[address, value])
            .map_err(Error::I2C)
    }
}
