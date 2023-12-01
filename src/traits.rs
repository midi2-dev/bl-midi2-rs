use crate::*;

pub trait Data {
    fn data(&self) -> &[u32];
}

pub trait ByteData {
    fn byte_data(&self) -> &[u8];
}

pub trait FromData<'a>: Sized {
    type Target;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target;
    fn validate_data(buffer: &'a [u32]) -> Result<()>;
    fn from_data(buffer: &'a [u32]) -> Result<Self::Target> {
        match Self::validate_data(buffer) {
            Ok(()) => Ok(Self::from_data_unchecked(buffer)),
            Err(e) => Err(e),
        }
    }
}

pub trait FromByteData<'a>: Sized {
    type Target;
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target;
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()>;
    fn from_byte_data(buffer: &'a [u8]) -> Result<Self::Target> {
        match Self::validate_byte_data(buffer) {
            Ok(()) => Ok(Self::from_byte_data_unchecked(buffer)),
            Err(e) => Err(e),
        }
    }
}

pub trait IntoOwned {
    type Owned;
    fn into_owned(self) -> Self::Owned;
}

pub trait Grouped {
    fn group(&self) -> u4;
}

pub trait GroupedBuilder {
    fn group(self, v: u4) -> Self;
}

pub trait Sysex<'a, 'b: 'a> {
    type PayloadIterator: core::iter::Iterator<Item = u8>;
    fn payload(&'b self) -> Self::PayloadIterator;
}

pub trait Streamed {
    fn stream_id(&self) -> u8;
}

pub trait StreamedBuilder {
    fn stream_id(self, v: u8) -> Self;
}
