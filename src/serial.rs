use core::future::Future;

/// Read half of a serial interface
pub trait AsyncRead {
    /// Read error
    type Error;
    /// Read byte future for polling on completion
    type ReadByteFuture<'f>: Future<Output = Result<u8, Self::Error>>;
    /// Read future for polling on completion
    type ReadFuture<'f>: Future<Output = Result<(), Self::Error>>;

    /// Reads a single byte from the serial interface
    fn async_read_byte(&mut self) -> Self::ReadByteFuture<'_>;

    /// Reads an array of bytes from the serial interface
    fn async_read<'a>(&'a mut self, data: &'a mut [u8]) -> Self::ReadFuture<'a>;
}

/// Write half of a serial interface
pub trait AsyncWrite {
    /// Write error
    type Error;
    /// Write byte future for polling on completion
    type WriteByteFuture<'f>: Future<Output = Result<(), Self::Error>>;
    /// Write future for polling on completion
    type WriteFuture<'f>: Future<Output = Result<(), Self::Error>>;
    /// Flush future for polling on completion
    type FlushFuture<'f>: Future<Output = Result<(), Self::Error>>;

    /// Writes a single byte to the serial interface
    /// When the future completes, data may not be fully transmitted.
    /// Call `flush` to ensure that no data is left buffered.
    fn async_write_byte(&mut self, byte: u8) -> Self::WriteByteFuture<'_>;

    /// Writes an array of bytes to the serial interface
    /// When the future completes, data may not be fully transmitted.
    /// Call `flush` to ensure that no data is left buffered.
    fn async_write<'a>(&'a mut self, data: &'a [u8]) -> Self::WriteFuture<'a>;

    /// Ensures that none of the previously written words are still buffered
    fn async_flush(&mut self) -> Self::FlushFuture<'_>;
}

pub mod read {
    use crate::serial::AsyncRead;
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    /// Marker trait to opt into default async read implementation
    ///
    /// Implementers of `embedded-hal::serial::Read` can implement this marker trait
    /// for their type. Doing so will automatically provide the default
    /// implementation of [`serial::AsyncRead`] for the type.
    ///
    /// [`serial::AsyncRead`]: ../trait.AsyncRead.html
    pub trait Default: embedded_hal::serial::Read<u8> {}

    impl<S: Default + 'static> AsyncRead for S {
        type Error = S::Error;
        type ReadByteFuture<'f> = DefaultReadByteFuture<'f, S>;
        type ReadFuture<'f> = DefaultReadFuture<'f, S>;

        fn async_read_byte(&mut self) -> Self::ReadByteFuture<'_> {
            DefaultReadByteFuture { serial: self }
        }

        fn async_read<'a>(&'a mut self, data: &'a mut [u8]) -> Self::ReadFuture<'a> {
            DefaultReadFuture {
                serial: self,
                data,
                offset: 0,
            }
        }
    }

    pub struct DefaultReadByteFuture<'a, S> {
        serial: &'a mut S,
    }

    impl<'a, S: Default> Future for DefaultReadByteFuture<'a, S> {
        type Output = Result<u8, S::Error>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match self.serial.read() {
                Ok(byte) => Poll::Ready(Ok(byte)),
                Err(nb::Error::Other(e)) => Poll::Ready(Err(e)),
                Err(nb::Error::WouldBlock) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
    }

    pub struct DefaultReadFuture<'a, S> {
        serial: &'a mut S,
        data: &'a mut [u8],
        offset: usize,
    }

    impl<'a, S: Default> Future for DefaultReadFuture<'a, S> {
        type Output = Result<(), S::Error>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            while self.offset < self.data.len() {
                match self.serial.read() {
                    Ok(byte) => {
                        let offset = self.offset; // Stupid Rust
                        self.data[offset] = byte;
                        self.offset += 1;
                        continue;
                    }
                    Err(nb::Error::Other(e)) => {
                        return Poll::Ready(Err(e));
                    }
                    Err(nb::Error::WouldBlock) => {
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
                }
            }
            Poll::Ready(Ok(()))
        }
    }
}

pub mod write {
    use crate::serial::AsyncWrite;
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    /// Marker trait to opt into default async write implementation
    ///
    /// Implementers of `embedded-hal::serial::Write` can implement this marker trait
    /// for their type. Doing so will automatically provide the default
    /// implementation of [`serial::AsyncWrite`] for the type.
    ///
    /// [`serial::AsyncWrite`]: ../trait.AsyncWrite.html
    pub trait Default: embedded_hal::serial::Write<u8> {}

    impl<S: Default + 'static> AsyncWrite for S {
        type Error = S::Error;
        type WriteByteFuture<'f> = DefaultWriteByteFuture<'f, S>;
        type WriteFuture<'f> = DefaultWriteFuture<'f, S>;
        type FlushFuture<'f> = DefaultFlushFuture<'f, S>;

        fn async_write_byte(&mut self, byte: u8) -> Self::WriteByteFuture<'_> {
            DefaultWriteByteFuture { serial: self, byte }
        }

        fn async_write<'a>(&'a mut self, data: &'a [u8]) -> DefaultWriteFuture<'a, S> {
            DefaultWriteFuture { serial: self, data }
        }

        fn async_flush(&mut self) -> DefaultFlushFuture<'_, S> {
            DefaultFlushFuture { serial: self }
        }
    }

    pub struct DefaultWriteByteFuture<'a, S> {
        serial: &'a mut S,
        byte: u8,
    }

    impl<'a, S: Default> Future for DefaultWriteByteFuture<'a, S> {
        type Output = Result<(), S::Error>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let byte = self.byte;
            match self.serial.write(byte) {
                Ok(()) => Poll::Ready(Ok(())),
                Err(nb::Error::Other(e)) => Poll::Ready(Err(e)),
                Err(nb::Error::WouldBlock) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
    }

    pub struct DefaultWriteFuture<'a, S> {
        serial: &'a mut S,
        data: &'a [u8],
    }

    impl<'a, S: Default> Future for DefaultWriteFuture<'a, S> {
        type Output = Result<(), S::Error>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            while let Some(byte) = self.data.first() {
                match self.serial.write(*byte) {
                    Ok(()) => {
                        self.data = &self.data[1..];
                        continue;
                    }
                    Err(nb::Error::Other(e)) => return Poll::Ready(Err(e)),
                    Err(nb::Error::WouldBlock) => {
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
                }
            }
            Poll::Ready(Ok(()))
        }
    }

    pub struct DefaultFlushFuture<'a, S> {
        serial: &'a mut S,
    }

    impl<'a, S: Default> Future for DefaultFlushFuture<'a, S> {
        type Output = Result<(), S::Error>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match self.serial.flush() {
                Ok(()) => Poll::Ready(Ok(())),
                Err(nb::Error::Other(e)) => Poll::Ready(Err(e)),
                Err(nb::Error::WouldBlock) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
    }
}
