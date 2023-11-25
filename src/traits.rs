use crate::*;

pub(crate) trait DataPrivate<B: Buffer> {
    fn data(&self) -> &[B::Data];
}

pub trait Data {
    fn data(&self) -> &[u32];
}

pub(crate) trait FromDataPrivate<'a, B: Buffer>: Sized {
    type Target;
    fn from_data_unchecked(buffer: &'a [B::Data]) -> Self::Target;
    fn validate_data(buffer: &'a [B::Data]) -> Result<()>;
    fn from_data(buffer: &'a [B::Data]) -> Result<Self::Target> {
        match Self::validate_data(buffer) {
            Ok(()) => Ok(Self::from_data_unchecked(buffer)),
            Err(e) => Err(e),
        }
    }
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

pub trait ToOwned {
    type Owned;
    fn to_owned(self) -> Self::Owned;
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
