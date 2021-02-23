use core::convert::TryFrom;
use core::future::Future;

mod sealed {
    pub trait I2cAddressType {}
}
use sealed::I2cAddressType;

#[derive(Debug)]
pub struct AddressRangeError;

/// A 7-bit I2C address
pub struct I2cAddress7Bit(u8);

impl TryFrom<u8> for I2cAddress7Bit {
    type Error = AddressRangeError;

    #[inline]
    fn try_from(address: u8) -> Result<Self, Self::Error> {
        if address < 0x80 {
            Ok(I2cAddress7Bit(address))
        } else {
            Err(AddressRangeError)
        }
    }
}

impl From<I2cAddress7Bit> for u8 {
    #[inline(always)]
    fn from(address: I2cAddress7Bit) -> Self {
        address.0
    }
}

impl I2cAddressType for I2cAddress7Bit {}

/// A 10-bit I2C address
pub struct I2cAddress10Bit(u16);

impl TryFrom<u16> for I2cAddress10Bit {
    type Error = AddressRangeError;

    #[inline]
    fn try_from(address: u16) -> Result<Self, Self::Error> {
        if address < 0x400 {
            Ok(I2cAddress10Bit(address))
        } else {
            Err(AddressRangeError)
        }
    }
}

impl From<I2cAddress10Bit> for u16 {
    #[inline(always)]
    fn from(address: I2cAddress10Bit) -> Self {
        address.0
    }
}

impl I2cAddressType for I2cAddress10Bit {}

/// I2C transfer. An `async` version of [`i2c::blocking::WriteRead`]
///
/// [`i2c::blocking::WriteRead`]: https://docs.rs/embedded-hal/0.2.4/embedded_hal/blocking/i2c/trait.WriteRead.html
pub trait AsyncI2cTransfer<A: I2cAddressType> {
    /// Transfer error
    type Error;

    /// Transfer future for polling on completion
    type TransferFuture<'f>: Future<Output = Result<(), Self::Error>>;

    /// Sends bytes to the slave. Returns the bytes received from the slave
    fn async_transfer<'a>(
        &'a mut self,
        address: A,
        tx_data: &'a [u8],
        rx_data: &'a mut [u8],
    ) -> Self::TransferFuture<'a>;
}

/// I2C write
pub trait AsyncI2cWrite<A: I2cAddressType> {
    /// Write error
    type Error;

    /// Write future for polling on completion
    type WriteFuture<'f>: Future<Output = Result<(), Self::Error>>;

    /// Sends bytes to the slave, ignoring all the incoming bytes
    fn async_write<'a>(&'a mut self, address: A, data: &'a [u8]) -> Self::WriteFuture<'_>;
}
