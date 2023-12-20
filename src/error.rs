#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    BufferOverflow,
    InvalidData,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}
