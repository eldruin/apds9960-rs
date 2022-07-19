#[cfg(feature = "async")]
use crate::{Apds9960Async, I2cAsync};
#[cfg(feature = "nb")]
use crate::{Apds9960, Write, WriteRead};
use {
    crate::register::{Config2, Enable, Status},
    crate::{BitFlags, Error, Register, DEV_ADDR},
};

/// Proximity.
#[maybe_async_cfg::maybe(
    sync(feature = "nb", keep_self),
    async(feature = "async", idents(Write(async = "I2cAsync")))
)]
impl<I2C, E> Apds9960<I2C>
where
    I2C: Write<Error = E>,
{
    /// Enable proximity detection
    pub async fn enable_proximity(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PEN, true).await
    }

    /// Disable proximity detection
    pub async fn disable_proximity(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PEN, false).await
    }

    /// Enable proximity interrupt generation
    pub async fn enable_proximity_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PIEN, true).await
    }

    /// Disable proximity interrupt generation
    pub async fn disable_proximity_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_enable(Enable::PIEN, false).await
    }

    /// Enable proximity saturation interrupt generation
    pub async fn enable_proximity_saturation_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config2(Config2::PSIEN, true).await
    }

    /// Disable proximity saturation interrupt generation
    pub async fn disable_proximity_saturation_interrupts(&mut self) -> Result<(), Error<E>> {
        self.set_flag_config2(Config2::PSIEN, false).await
    }

    /// Set the proximity interrupt low threshold.
    pub async fn set_proximity_low_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        self.write_register(Register::PILT, threshold).await
    }

    /// Set the proximity interrupt high threshold.
    pub async fn set_proximity_high_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        self.write_register(Register::PIHT, threshold).await
    }

    /// Set the proximity up/right photodiode offset.
    pub async fn set_proximity_up_right_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.write_register(Register::POFFSET_UR, offset as u8)
            .await
    }

    /// Set the proximity down/left photodiode offset.
    pub async fn set_proximity_down_left_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.write_register(Register::POFFSET_DL, offset as u8)
            .await
    }

    /// Set the proximity up/right and down/left photodiode offset.
    pub async fn set_proximity_offsets(
        &mut self,
        offset_up_right: i8,
        offset_down_left: i8,
    ) -> Result<(), Error<E>> {
        self.i2c
            .write(
                DEV_ADDR,
                &[
                    Register::POFFSET_UR,
                    offset_up_right as u8,
                    offset_down_left as u8,
                ],
            )
            .await
            .map_err(Error::I2C)
    }

    /// Clear proximity interrupt.
    pub async fn clear_proximity_interrupt(&mut self) -> Result<(), Error<E>> {
        self.touch_register(Register::PICLEAR).await
    }
}
#[maybe_async_cfg::maybe(
    sync(feature = "nb", keep_self),
    async(feature = "async", idents(WriteRead(async = "I2cAsync")))
)]
impl<I2C, E> Apds9960<I2C>
where
    I2C: WriteRead<Error = E>,
{
    #[maybe_async_cfg::only_if(sync)]
    /// Read the proximity sensor data.
    ///
    /// Blocks as long as the data is not ready.
    pub fn read_proximity(&mut self) -> nb::Result<u8, Error<E>> {
        if !self.is_proximity_data_valid().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        self.read_register(Register::PDATA)
            .map_err(nb::Error::Other)
    }

    #[maybe_async_cfg::only_if(async)]
    /// Read the proximity sensor data.
    ///
    /// Blocks as long as the data is not ready.
    pub async fn read_proximity(&mut self) -> Result<u8, Error<E>> {
        while !self.is_proximity_data_valid().await? {}
        self.read_register(Register::PDATA).await
    }

    /// Read whether the proximity sensor data is valid.
    ///
    /// This is checked internally in `read_proximity()` as well.
    #[allow(clippy::wrong_self_convention)]
    pub async fn is_proximity_data_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS).await?;
        Ok(Status::create(status).is(Status::PVALID, true))
    }
}
