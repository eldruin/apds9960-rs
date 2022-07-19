#[maybe_async_cfg::maybe(
    sync(feature = "nb", keep_self),
    async(feature = "async", self = "asynch", idents(Apds9960,I2cWriteRead))
)]
mod blocking {
    use crate::I2cWriteRead;
    use crate::{register::GStatus, Apds9960, BitFlags, Error, Register};

    /// Gesture data reading.
    impl<I2C, E> Apds9960<I2C>
    where
        I2C: I2cWriteRead<E, Error = E>,
    {
        /// Read the amount of available data in the gesture FIFO registers.
        pub async fn read_gesture_data_level(&mut self) -> Result<u8, Error<E>> {
            self.read_register(Register::GFLVL).await
        }

        /// Read whether there is valid gesture data available.
        #[allow(clippy::wrong_self_convention)]
        pub async fn is_gesture_data_valid(&mut self) -> Result<bool, Error<E>> {
            let status = self.read_register(Register::GSTATUS).await?;
            Ok(GStatus::create(status).is(GStatus::GVALID, true))
        }

        /// Read whether the gesture data has overflown.
        #[allow(clippy::wrong_self_convention)]
        pub async fn has_gesture_data_overflown(&mut self) -> Result<bool, Error<E>> {
            let status = self.read_register(Register::GSTATUS).await?;
            Ok(GStatus::create(status).is(GStatus::GFOV, true))
        }

        /// Read gesture data.
        ///
        /// Will read the gesture data up to the minimum of: gesture data level, array size.
        /// Make sure to provide an array with at least the number of elements returned by the
        /// `read_gesture_data_level()` method multiplied by 4.
        ///
        /// The data contents will be organized as follows:
        /// `[up_dataset0, down_dataset0, left_dataset0, right_dataset0,
        ///   up_dataset1, down_dataset1, left_dataset1, right_dataset1, ...]`
        ///
        /// Blocks as long as not enough data is available.
        #[maybe_async_cfg::only_if(sync)]
        pub fn read_gesture_data(&mut self, data: &mut [u8]) -> nb::Result<(), Error<E>> {
            if !self.is_gesture_data_valid().map_err(nb::Error::Other)? {
                return Err(nb::Error::WouldBlock);
            }
            let level = self.read_gesture_data_level().map_err(nb::Error::Other)?;
            let byte_count = core::cmp::min(data.len(), 4 * level as usize);
            self.read_data(Register::GFIFO_U, &mut data[..byte_count])
                .map_err(nb::Error::Other)?;
            Ok(())
        }
        /// Read gesture data.
        ///
        /// Will read the gesture data up to the minimum of: gesture data level, array size.
        /// Make sure to provide an array with at least the number of elements returned by the
        /// `read_gesture_data_level()` method multiplied by 4.
        ///
        /// The data contents will be organized as follows:
        /// `[up_dataset0, down_dataset0, left_dataset0, right_dataset0,
        ///   up_dataset1, down_dataset1, left_dataset1, right_dataset1, ...]`
        ///
        /// Blocks as long as not enough data is available.
        #[maybe_async_cfg::only_if(async)]
        pub async fn read_gesture_data(&mut self, data: &mut [u8]) -> Result<(), Error<E>> {
            while !self.is_gesture_data_valid().await? {}
            let level = self.read_gesture_data_level().await?;
            let byte_count = core::cmp::min(data.len(), 4 * level as usize);
            self.read_data(Register::GFIFO_U, &mut data[..byte_count])
                .await?;
            Ok(())
        }
    }
}
