//! Delays

use core::future::Future;

/// Millisecond delay
///
/// `UXX` denotes the range type of the delay time. `UXX` can be `u8`, `u16`, etc. A single type can
/// implement this trait for different types of `UXX`.
pub trait AsyncDelayMs<UXX> {
    /// Delay future for polling on completion
    type DelayFuture<'f>: Future<Output = ()>;

    /// Pauses execution for `ms` milliseconds
    fn async_delay_ms(&mut self, ms: UXX) -> Self::DelayFuture<'_>;
}

/// Microsecond delay
///
/// `UXX` denotes the range type of the delay time. `UXX` can be `u8`, `u16`, etc. A single type can
/// implement this trait for different types of `UXX`.
pub trait AsyncDelayUs<UXX> {
    /// Delay future for polling on completion
    type DelayFuture<'f>: Future<Output = ()>;

    /// Pauses execution for `us` microseconds
    fn async_delay_us(&mut self, us: UXX) -> Self::DelayFuture<'_>;
}

/// Implement `AsyncDelayMs<u16>`, `AsyncDelayMs<u8>` and `AsyncDelayMs<i32>`
/// based on the `AsyncDelayMs<u32>` implementation.
#[macro_export]
macro_rules! impl_delay_ms_for_ms_u32 {
    ($delay:ty) => {
        impl AsyncDelayMs<u16> for $delay {
            type DelayFuture<'f> = <$delay as AsyncDelayMs<u32>>::DelayFuture<'f>;

            #[inline(always)]
            fn async_delay_ms(&mut self, ms: u16) -> Self::DelayFuture<'_> {
                self.async_delay_ms(u32::from(ms))
            }
        }

        impl AsyncDelayMs<u8> for $delay {
            type DelayFuture<'f> = <$delay as AsyncDelayMs<u32>>::DelayFuture<'f>;

            #[inline(always)]
            fn async_delay_ms(&mut self, ms: u8) -> Self::DelayFuture<'_> {
                self.async_delay_ms(u32::from(ms))
            }
        }

        impl AsyncDelayMs<i32> for $delay {
            type DelayFuture<'f> = <$delay as AsyncDelayMs<u32>>::DelayFuture<'f>;

            #[inline(always)]
            fn async_delay_ms(&mut self, ms: i32) -> Self::DelayFuture<'_> {
                assert!(ms >= 0);
                self.async_delay_ms(ms as u32)
            }
        }
    };
}

/// Implement `AsyncDelayUs<u16>`, `AsyncDelayUs<u8>` and `AsyncDelayUs<i32>`
/// based on the `AsyncDelayUs<u32>` implementation.
#[macro_export]
macro_rules! impl_delay_us_for_us_u32 {
    ($delay:ty) => {
        impl AsyncDelayUs<u16> for $delay {
            type DelayFuture<'f> = <$delay as AsyncDelayUs<u32>>::DelayFuture<'f>;

            #[inline(always)]
            fn async_delay_us(&mut self, us: u16) -> Self::DelayFuture<'_> {
                self.async_delay_us(u32::from(us))
            }
        }

        impl AsyncDelayUs<u8> for $delay {
            type DelayFuture<'f> = <$delay as AsyncDelayUs<u32>>::DelayFuture<'f>;

            #[inline(always)]
            fn async_delay_us(&mut self, us: u8) -> Self::DelayFuture<'_> {
                self.async_delay_us(u32::from(us))
            }
        }

        impl AsyncDelayUs<i32> for $delay {
            type DelayFuture<'f> = <$delay as AsyncDelayUs<u32>>::DelayFuture<'f>;

            #[inline(always)]
            fn async_delay_us(&mut self, us: i32) -> Self::DelayFuture<'_> {
                assert!(us >= 0);
                self.async_delay_us(us as u32)
            }
        }
    };
}

/// Implement `AsyncDelayUs<u32>`, `AsyncDelayUs<u16>`, `AsyncDelayUs<u8>` and `AsyncDelayUs<i32>`
/// based on the `AsyncDelayUs<u64>` implementation.
#[macro_export]
macro_rules! impl_delay_us_for_us_u64 {
    ($delay:ty) => {
        impl AsyncDelayUs<u32> for $delay {
            type DelayFuture<'f> = <$delay as AsyncDelayUs<u64>>::DelayFuture<'f>;

            #[inline(always)]
            fn async_delay_us(&mut self, us: u32) -> Self::DelayFuture<'_> {
                self.async_delay_us(us as u64)
            }
        }

        $crate::impl_delay_us_for_us_u32!($delay);
    };
}

/// Implement `AsyncDelayMs<u32>`, `AsyncDelayMs<u16>`, `AsyncDelayMs<u8>` and `AsyncDelayMs<i32>`
/// based on the `AsyncDelayUs<u64>` implementation.
#[macro_export]
macro_rules! impl_delay_ms_for_us_u64 {
    ($delay:ty) => {
        impl AsyncDelayMs<u32> for $delay {
            type DelayFuture<'f> = <$delay as AsyncDelayUs<u64>>::DelayFuture<'f>;

            fn async_delay_ms(&mut self, ms: u32) -> Self::DelayFuture<'_> {
                self.async_delay_us((ms as u64) * 1000)
            }
        }

        $crate::impl_delay_ms_for_ms_u32!($delay);
    };
}
