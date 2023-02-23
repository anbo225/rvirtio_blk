

pub mod transport;

mod hal;
mod queue;

use core::fmt;
use std::fmt::{Formatter, Display};

pub use self::hal::{PhysAddr};

/// The page size in bytes supported by the library (4 KiB).
pub const PAGE_SIZE: usize = 0x1000;

/// The type returned by driver methods.
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The error type of VirtIO drivers.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// There are not enough descriptors available in the virtqueue, try again later.
    QueueFull,
    /// The device is not ready.
    NotReady,
    /// The device used a different descriptor chain to the one we were expecting.
    WrongToken,
    /// The queue is already in use.
    AlreadyUsed,
    /// Invalid parameter.
    InvalidParam,
    /// Failed to alloc DMA memory.
    DmaError,
    /// I/O Error
    IoError,
    /// The request was not supported by the device.
    Unsupported,
    /// The config space advertised by the device is smaller than the driver expected.
    ConfigSpaceTooSmall,
    /// The device doesn't have any config space, but the driver expects some.
    ConfigSpaceMissing,
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::QueueFull => write!(f, "Virtqueue is full"),
            Self::NotReady => write!(f, "Device not ready"),
            Self::WrongToken => write!(
                f,
                "Device used a different descriptor chain to the one we were expecting"
            ),
            Self::AlreadyUsed => write!(f, "Virtqueue is already in use"),
            Self::InvalidParam => write!(f, "Invalid parameter"),
            Self::DmaError => write!(f, "Failed to allocate DMA memory"),
            Self::IoError => write!(f, "I/O Error"),
            Self::Unsupported => write!(f, "Request not supported by device"),
            Self::ConfigSpaceTooSmall => write!(
                f,
                "Config space advertised by the device is smaller than expected"
            ),
            Self::ConfigSpaceMissing => {
                write!(
                    f,
                    "The device doesn't have any config space, but the driver expects some"
                )
            }
        }
    }
}
