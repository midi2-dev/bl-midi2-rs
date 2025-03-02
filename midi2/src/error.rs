#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    BufferOverflow,
    InvalidData(InvalidData),
}

impl core::convert::From<BufferOverflow> for Error {
    fn from(_value: BufferOverflow) -> Self {
        Error::BufferOverflow
    }
}

impl core::convert::From<InvalidData> for Error {
    fn from(value: InvalidData) -> Self {
        Error::InvalidData(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BufferOverflow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidData(pub &'static str);

#[cfg(feature = "std")]
impl std::error::Error for BufferOverflow {}

#[cfg(feature = "std")]
impl std::fmt::Display for BufferOverflow {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidData {}

#[cfg(feature = "std")]
impl std::fmt::Display for InvalidData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl core::convert::From<crate::traits::SysexTryResizeError> for BufferOverflow {
    fn from(_value: crate::traits::SysexTryResizeError) -> Self {
        BufferOverflow
    }
}
