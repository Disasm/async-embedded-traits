//! The prelude is a collection of all the traits in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

pub use crate::delay::{
    AsyncDelayMs as _async_embedded_traits_delay_AsyncDelayMs,
    AsyncDelayUs as _async_embedded_traits_delay_AsyncDelayUs,
};

pub use crate::serial::{
    AsyncRead as _async_embedded_traits_serial_AsyncRead,
    AsyncWrite as _async_embedded_traits_serial_AsyncWrite,
};

pub use crate::spi::{
    AsyncTransfer as _async_embedded_traits_spi_AsyncTransfer,
    AsyncWrite as _async_embedded_traits_spi_AsyncWrite,
    AsyncWriteIter as _async_embedded_traits_spi_AsyncWriteIter,
};
