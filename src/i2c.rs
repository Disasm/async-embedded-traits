use core::future::Future;

/// I2C transfer. An `async` version of [`i2c::blocking::WriteRead`]
///
/// [`i2c::blocking::WriteRead`]: https://docs.rs/embedded-hal/0.2.4/embedded_hal/blocking/i2c/trait.WriteRead.html
pub trait AsyncI2cTransfer {
    /// Transfer error
    type Error;

    /// Transfer future for polling on completion
    type TransferFuture<'f>: Future<Output = Result<(), Self::Error>>;

    /// Sends bytes to the slave. Returns the bytes received from the slave
    fn async_transfer<'a>(
        &'a mut self,
        address: u16,
        tx_data: &'a [u8],
        rx_data: &'a mut [u8],
    ) -> Self::TransferFuture<'a>;
}

/// I2C write
pub trait AsyncI2cWrite {
    /// Write error
    type Error;

    /// Write future for polling on completion
    type WriteFuture<'f>: Future<Output = Result<(), Self::Error>>;

    /// Sends bytes to the slave, ignoring all the incoming bytes
    fn async_write<'a>(
        &'a mut self,
        address: u16,
        data: &'a [u8],
    ) -> Self::WriteFuture<'_>;
}
