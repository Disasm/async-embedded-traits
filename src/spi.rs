use core::future::Future;

/// SPI transfer
pub trait AsyncTransfer {
    /// Transfer error
    type Error;
    /// Transfer future for polling on completion
    type TransferFuture<'t>: Future<Output=Result<(), Self::Error>>;

    /// Sends bytes to the slave. Returns the bytes received from the slave
    fn async_transfer<'a>(&'a mut self, data: &'a mut [u8]) -> Self::TransferFuture<'a>;
}

/// SPI write
pub trait AsyncWrite {
    /// Write error
    type Error;
    /// Write future for polling on completion
    type WriteFuture<'t>: Future<Output=Result<(), Self::Error>>;

    /// Sends bytes to the slave, ignoring all the incoming bytes
    fn async_write<'a>(&'a mut self, data: &'a [u8]) -> Self::WriteFuture<'_>;
}

/// SPI write (iterator version)
pub trait AsyncWriteIter {
    /// Write error
    type Error;
    /// Write future for polling on completion
    type WriteIterFuture<'t>: Future<Output=Result<(), Self::Error>>;

    /// Sends bytes to the slave, ignoring all the incoming bytes
    fn async_write_iter<'a>(&'a mut self, data: &'a mut dyn Iterator<Item=u8>) -> Self::WriteIterFuture<'_>;
}

pub mod transfer {
    use super::AsyncTransfer;
    use core::future::Future;
    use core::task::{Context, Poll};
    use core::pin::Pin;

    /// Marker trait to opt into default async transfer implementation
    ///
    /// Implementers of `embedded-hal::spi::FullDuplex<u8>` can implement this marker trait
    /// for their type. Doing so will automatically provide the default
    /// implementation of [`spi::AsyncTransfer`] for the type.
    ///
    /// [`spi::AsyncTransfer`]: ../trait.AsyncTransfer.html
    pub trait Default: embedded_hal::spi::FullDuplex<u8> {}

    impl<S: Default + 'static> AsyncTransfer for S {
        type Error = S::Error;
        type TransferFuture<'t> = DefaultTransferFuture<'t, S>;

        fn async_transfer<'a>(&'a mut self, data: &'a mut [u8]) -> Self::TransferFuture<'a> {
            DefaultTransferFuture {
                spi: self,
                data,
                offset: 0,
                state: State::Sending
            }
        }
    }

    enum State {
        Sending,
        Receiving,
    }

    pub struct DefaultTransferFuture<'a, S> {
        spi: &'a mut S,
        data: &'a mut [u8],
        offset: usize,
        state: State,
    }

    impl<'a, S: Default> Future for DefaultTransferFuture<'a, S> {
        type Output = Result<(), S::Error>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            while self.offset < self.data.len() {
                match self.state {
                    State::Sending => {
                        let byte = self.data[self.offset];
                        match self.spi.send(byte) {
                            Ok(()) => {
                                self.state = State::Receiving;
                                continue;
                            },
                            Err(nb::Error::Other(e)) => {
                                return Poll::Ready(Err(e));
                            },
                            Err(nb::Error::WouldBlock) => {
                                cx.waker().wake_by_ref();
                                return Poll::Pending;
                            }
                        }
                    },
                    State::Receiving => {
                        match self.spi.read() {
                            Ok(byte) => {
                                let offset = self.offset;
                                self.data[offset] = byte;
                                self.offset += 1;
                                self.state = State::Sending;
                                continue;
                            },
                            Err(nb::Error::Other(e)) => {
                                return Poll::Ready(Err(e));
                            },
                            Err(nb::Error::WouldBlock) => {
                                cx.waker().wake_by_ref();
                                return Poll::Pending;
                            }
                        }
                    },
                }
            }
            Poll::Ready(Ok(()))
        }
    }
}
