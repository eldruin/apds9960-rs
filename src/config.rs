use hal::blocking::i2c;
use {
    register::{Enable, GConfig1, GConfig4},
    Apds9960, BitFlags, Error, GestureDataThreshold, Register, DEV_ADDR,
};

macro_rules! impl_set_flag_reg {
    ($method:ident, $reg:ident) => {
        fn $method(&mut self, flag: u8, value: bool) -> Result<(), Error<E>> {
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
    /// Create new instance of the APDS9960 device.
    pub fn new(i2c: I2C) -> Self {
        Apds9960 {
            i2c,
            enable: Enable::default(),
            gconfig1: GConfig1::default(),
            gconfig4: GConfig4::default(),
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Turn power on.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PON, true)
    }

    /// Deactivate everything and put the device to sleep.
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::ALL, false)
    }

    /// Enable proximity detection
    pub fn enable_proximity(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PEN, true)
    }

    /// Disable proximity detection
    pub fn disable_proximity(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PEN, false)
    }

    /// Enable gesture detection
    pub fn enable_gesture(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::GEN, true)
    }

    /// Disable gesture detection
    pub fn disable_gesture(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::GEN, false)
    }

    /// Enable gesture mode.
    ///
    /// This can be automatically enabled (depending on proximity thresholds)
    /// and disabled (see GMODE on datasheet).
    pub fn enable_gesture_mode(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config4(GConfig4::GMODE, true)
    }

    /// Disable gesture mode.
    ///
    /// This can be automatically enabled (depending on proximity thresholds)
    /// and disabled (see GMODE on datasheet).
    pub fn disable_gesture_mode(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config4(GConfig4::GMODE, false)
    }

    /// Enable gesture interrupt generation
    pub fn enable_gesture_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config4(GConfig4::GIEN, true)
    }

    /// Disable gesture interrupt generation
    pub fn disable_gesture_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config4(GConfig4::GIEN, false)
    }

    /// Set the threshold of amount of available data in the gesture FIFO registers.
    pub fn set_gesture_data_level_threshold(
        &mut self,
        threshold: GestureDataThreshold,
    ) -> Result<(), Error<E>> {
        use GestureDataThreshold as GDTH;
        let flags;
        match threshold {
            GDTH::Th1 => flags = (false, false),
            GDTH::Th4 => flags = (false, true),
            GDTH::Th8 => flags = (true, false),
            GDTH::Th16 => flags = (true, true),
        }
        let new = self
            .gconfig1
            .with(GConfig1::GFIFOTH1, flags.0)
            .with(GConfig1::GFIFOTH0, flags.1);
        self.config_register(&new)?;
        self.gconfig1 = new;
        Ok(())
    }

    /// Set the gesture proximity entry threshold.
    pub fn set_gesture_proximity_entry_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        self.write_register(Register::GPENTH, threshold)
    }

    /// Set the gesture proximity exit threshold.
    pub fn set_gesture_proximity_exit_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        self.write_register(Register::GPEXTH, threshold)
    }

    /// Set the gesture up offset.
    pub fn set_gesture_up_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.write_register(Register::GOFFSET_U, offset as u8)
    }

    /// Set the gesture down offset.
    pub fn set_gesture_down_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.write_register(Register::GOFFSET_D, offset as u8)
    }

    /// Set the gesture left offset.
    pub fn set_gesture_left_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.write_register(Register::GOFFSET_L, offset as u8)
    }

    /// Set the gesture right offset.
    pub fn set_gesture_right_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.write_register(Register::GOFFSET_R, offset as u8)
    }

    /// Set the gesture up, down, left and right offsets.
    pub fn set_gesture_offsets(
        &mut self,
        offset_up: i8,
        offset_down: i8,
        offset_left: i8,
        offset_right: i8,
    ) -> Result<(), Error<E>> {
        self.i2c
            .write(
                DEV_ADDR,
                &[
                    Register::GOFFSET_U,
                    offset_up as u8,
                    offset_down as u8,
                    offset_left as u8,
                    offset_right as u8,
                ],
            )
            .map_err(Error::I2C)
    }

    impl_set_flag_reg!(set_flag_enable, enable);
    impl_set_flag_reg!(set_flag_config4, gconfig4);

    fn config_register<T: BitFlags>(&mut self, reg: &T) -> Result<(), Error<E>> {
        self.write_register(T::ADDRESS, reg.value())
    }

    fn write_register(&mut self, address: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEV_ADDR, &[address, value])
            .map_err(Error::I2C)
    }
}
