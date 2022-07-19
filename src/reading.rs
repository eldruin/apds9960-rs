#[maybe_async_cfg::maybe(
    sync(feature = "nb", keep_self),
    async(feature = "async", self = "asynch", idents(Apds9960,I2cWrite,I2cWriteRead))
)]
mod blocking {
    use crate::I2cWriteRead;
    use crate::{Apds9960, Error, Register, DEV_ADDR};

    impl<I2C, E> Apds9960<I2C>
    where
        I2C: I2cWriteRead<E, Error = E>,
    {
        /// Read the device ID.
        ///
        /// This is per default `0xAB`.
        pub async fn read_device_id(&mut self) -> Result<u8, Error<E>> {
            self.read_register(Register::ID).await
        }

        pub(crate) async fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
            let mut data = [0];
            self.read_data(register, &mut data).await?;
            Ok(data[0])
        }

        pub(crate) async fn read_data(
            &mut self,
            register: u8,
            data: &mut [u8],
        ) -> Result<(), Error<E>> {
            self.i2c
                .write_read(DEV_ADDR, &[register], data)
                .await
                .map_err(Error::I2C)
        }
    }
}
