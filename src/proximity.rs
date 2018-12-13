use hal::blocking::i2c;
use {
    register::{Config2, Enable},
    Apds9960, Error, Register, DEV_ADDR,
};


impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Enable proximity detection
    pub fn enable_proximity(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PEN, true)
    }

    /// Disable proximity detection
    pub fn disable_proximity(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PEN, false)
    }

    /// Enable proximity interrupt generation
    pub fn enable_proximity_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PIEN, true)
    }

    /// Disable proximity interrupt generation
    pub fn disable_proximity_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PIEN, false)
    }

    /// Enable proximity saturation interrupt generation
    pub fn enable_proximity_saturation_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config2(Config2::PSIEN, true)
    }

    /// Disable proximity saturation interrupt generation
    pub fn disable_proximity_saturation_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config2(Config2::PSIEN, false)
    }

    /// Set the proximity interrupt low threshold.
    pub fn set_proximity_low_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        self.write_register(Register::PILT, threshold)
    }

    /// Set the proximity interrupt high threshold.
    pub fn set_proximity_high_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        self.write_register(Register::PIHT, threshold)
    }

    /// Set the proximity up/right photodiode offset.
    pub fn set_proximity_up_right_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.write_register(Register::POFFSET_UR, offset as u8)
    }

    /// Set the proximity down/left photodiode offset.
    pub fn set_proximity_down_left_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.write_register(Register::POFFSET_DL, offset as u8)
    }

    /// Set the proximity up/right and down/left photodiode offset.
    pub fn set_proximity_offsets(&mut self, offset_up_right: i8, offset_down_left: i8) -> Result<(), Error<E>> {
        self.i2c
            .write(
                DEV_ADDR,
                &[
                    Register::POFFSET_UR,
                    offset_up_right as u8,
                    offset_down_left as u8,
                ],
            )
            .map_err(Error::I2C)
    }
}
