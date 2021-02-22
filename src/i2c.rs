use core::future::Future;

/// I2C transfer. An `async` version of [`i2c::blocking::WriteRead`]
///
/// [`i2c::blocking::WriteRead`]: https://docs.rs/embedded-hal/0.2.4/embedded_hal/blocking/i2c/trait.WriteRead.html
pub trait AsyncI2cTransfer {
    /// I2C slave address width: for 7-bit should be `u8`, for 10-bit `u16`
    type AddressWidth;

    /// Transfer error
    type Error;

    /// Transfer future for polling on completion
    type TransferFuture<'f>: Future<Output = Result<(), Self::Error>>;

    /// Sends bytes to the slave. Returns the bytes received from the slave
    fn async_transfer<'a>(
        &'a mut self,
        address: Self::AddressWidth,
        tx_data: &'a [u8],
        rx_data: &'a mut [u8],
    ) -> Self::TransferFuture<'a>;
}

/// I2C write
pub trait AsyncI2cWrite {
    /// I2C slave address width: for 7-bit should be `u8`, for 10-bit `u16`
    type AddressWidth;

    /// Write error
    type Error;

    /// Write future for polling on completion
    type WriteFuture<'f>: Future<Output = Result<(), Self::Error>>;

    /// Sends bytes to the slave, ignoring all the incoming bytes
    fn async_write<'a>(
        &'a mut self,
        address: Self::AddressWidth,
        data: &'a [u8],
    ) -> Self::WriteFuture<'_>;
}
