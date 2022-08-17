#[cfg(feature = "nb")]
use crate::{Apds9960, Write, WriteRead};
#[cfg(feature = "async")]
use crate::{Apds9960Async, I2cAsync};
use {
    crate::register::{Config2, Enable, Status},
    crate::{BitFlags, Error, LightData, Register},
};

#[cfg(feature = "nb")]
use nb::Result as NbResult;

/// Color and ambient light.
#[maybe_async_cfg::maybe(
    sync(feature = "nb", keep_self),
    async(
        feature = "async",
        idents(
            Write(async = "I2cAsync"),
            WriteRead(async = "I2cAsync"),
            NbResult(async = "Result")
        )
    )
)]
impl<I2C, E> Apds9960<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Enable color and ambient light detection.
    pub async fn enable_light(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::AEN, true).await
    }

    /// Disable color and ambient light detection.
    pub async fn disable_light(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::AEN, false).await
    }

    /// Set the color and ambient light integration time.
    ///
    /// The value parameter must be a 2's complement of the number of cycles.
    ///
    /// Per default this is set to `0xFF` (1 cycle) and each cycle has a fixed duration of 2.78ms.
    pub async fn set_light_integration_time(&mut self, value: u8) -> Result<(), Error<E>> {
        self.write_register(Register::ATIME, value).await
    }

    /// Enable ambient light interrupt generation.
    pub async fn enable_light_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::AIEN, true).await
    }

    /// Disable ambient light interrupt generation.
    pub async fn disable_light_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::AIEN, false).await
    }

    /// Enable clear channel ambient light saturation interrupt generation.
    pub async fn enable_light_saturation_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config2(Config2::CPSIEN, true).await
    }

    /// Disable clear channel ambient light saturation interrupt generation.
    pub async fn disable_light_saturation_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config2(Config2::CPSIEN, false).await
    }

    /// Set the clear channel ambient light interrupt low threshold.
    ///
    /// An interrupt will be generated if light interrupts are enabled and the clear data is less
    /// than this value.
    pub async fn set_light_low_threshold(&mut self, threshold: u16) -> Result<(), Error<E>> {
        self.write_double_register(Register::AILTL, threshold).await
    }

    /// Set the clear channel ambient light interrupt high threshold.
    ///
    /// An interrupt will be generated if light interrupts are enabled and the clear data is greater
    /// than this value.
    pub async fn set_light_high_threshold(&mut self, threshold: u16) -> Result<(), Error<E>> {
        self.write_double_register(Register::AIHTL, threshold).await
    }

    /// Clear ambient light interrupt.
    pub async fn clear_light_interrupt(&mut self) -> Result<(), Error<E>> {
        self.touch_register(Register::CICLEAR).await
    }

    /// Read the color / ambient light sensor data.
    ///
    /// Returns `nb::Error::WouldBlock` as long as the data is not ready.
    /// This clears the data ready flag.
    #[maybe_async_cfg::only_if(sync)]
    pub fn read_light(&mut self) -> nb::Result<LightData, Error<E>> {
        if !self.is_light_data_valid().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        let mut data = [0; 8];
        self.read_data(Register::CDATAL, &mut data)
            .map_err(nb::Error::Other)?;
        Ok(LightData {
            clear: (u16::from(data[1]) << 8) | u16::from(data[0]),
            red: (u16::from(data[3]) << 8) | u16::from(data[2]),
            green: (u16::from(data[5]) << 8) | u16::from(data[4]),
            blue: (u16::from(data[7]) << 8) | u16::from(data[6]),
        })
    }

    /// Read the color / ambient light sensor data.
    ///
    /// Blocks as long as the data is not ready.
    /// This clears the data ready flag.
    #[maybe_async_cfg::only_if(async)]
    pub async fn read_light(&mut self) -> NbResult<LightData, Error<E>> {
        while !self.is_light_data_valid().await? {}
        let mut data = [0; 8];
        self.read_data(Register::CDATAL, &mut data).await?;
        Ok(LightData {
            clear: (u16::from(data[1]) << 8) | u16::from(data[0]),
            red: (u16::from(data[3]) << 8) | u16::from(data[2]),
            green: (u16::from(data[5]) << 8) | u16::from(data[4]),
            blue: (u16::from(data[7]) << 8) | u16::from(data[6]),
        })
    }

    /// Read the color / ambient light sensor clear channel data.
    ///
    /// Blocks (asynchronously) or returns `nb::Error::WouldBlock` when the data is not ready.
    /// This clears the data ready flag.
    pub async fn read_light_clear(&mut self) -> NbResult<u16, Error<E>> {
        self.read_light_channel(Register::CDATAL).await
    }

    /// Read the color / ambient light sensor red channel data.
    ///
    /// Blocks (asynchronously) or returns `nb::Error::WouldBlock` when the data is not ready.
    /// This clears the data ready flag.
    pub async fn read_light_red(&mut self) -> NbResult<u16, Error<E>> {
        self.read_light_channel(Register::RDATAL).await
    }

    /// Read the color / ambient light sensor green channel data.
    ///
    /// Blocks (asynchronously) or returns `nb::Error::WouldBlock` when the data is not ready.
    /// This clears the data ready flag.
    pub async fn read_light_green(&mut self) -> NbResult<u16, Error<E>> {
        self.read_light_channel(Register::GDATAL).await
    }

    /// Read the color / ambient light sensor blue channel data.
    ///
    /// Blocks (asynchronously) or returns `nb::Error::WouldBlock` when the data is not ready.
    /// This clears the data ready flag.
    pub async fn read_light_blue(&mut self) -> NbResult<u16, Error<E>> {
        self.read_light_channel(Register::BDATAL).await
    }

    /// Read whether the color and ambient light sensor data is valid.
    ///
    /// This is checked internally in the `read_light_*()` methods as well.
    #[allow(clippy::wrong_self_convention)]
    pub async fn is_light_data_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS).await?;
        Ok(Status::create(status).is(Status::AVALID, true))
    }

    #[maybe_async_cfg::only_if(sync)]
    fn read_light_channel(&mut self, register: u8) -> nb::Result<u16, Error<E>> {
        if !self.is_light_data_valid().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        let mut data = [0; 2];
        self.read_data(register, &mut data)
            .map_err(nb::Error::Other)?;
        Ok((u16::from(data[1]) << 8) | u16::from(data[0]))
    }
    #[maybe_async_cfg::only_if(async)]
    async fn read_light_channel(&mut self, register: u8) -> Result<u16, Error<E>> {
        while !self.is_light_data_valid().await? {}
        let mut data = [0; 2];
        self.read_data(register, &mut data).await?;
        Ok((u16::from(data[1]) << 8) | u16::from(data[0]))
    }
}
