use crate::*;

pub trait Data<B: Buffer> {
    fn data(&self) -> &[B::Data];
}

pub trait FromData<'a, B: Buffer>: Sized {
    fn from_data_unchecked(buffer: &'a [B::Data]) -> Self;
    fn validate_data(buffer: &'a [B::Data]) -> Result<()>;
    fn from_data(buffer: &'a [B::Data]) -> Result<Self> {
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

pub trait Sysex<B: Buffer> {
    type PayloadIterator: core::iter::Iterator<Item = u8>;
    fn payload(&self) -> Self::PayloadIterator;
}

pub trait Streamed {
    fn stream_id(&self) -> u8;
}
