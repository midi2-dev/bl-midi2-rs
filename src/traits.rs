use crate::{util::BitOps, *};

pub trait Data {
    fn data(&self) -> &[u32];
}

pub trait ByteData {
    fn byte_data(&self) -> &[u8];
}

pub trait WriteByteData {
    fn write_byte_data<'a>(&self, buffer: &'a mut [u8]) -> &'a mut [u8];
}

pub trait TryWriteByteData {
    fn try_write_byte_data<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8]>;
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

pub trait TryIntoOwned {
    type Owned;
    fn try_into_owned(self) -> Result<Self::Owned>;
}

pub trait Grouped: Data {
    fn group(&self) -> u4 {
        self.data()[0].nibble(1)
    }
}

pub trait Channeled: Data {
    fn channel(&self) -> u4 {
        self.data()[0].nibble(3)
    }
}

pub trait Sysex<'a, 'b: 'a> {
    type PayloadIterator: core::iter::Iterator<Item = u8>;
    fn payload(&'b self) -> Self::PayloadIterator;
}

pub trait SysexBuilder {
    type ByteType;
    fn append_payload<I: core::iter::Iterator<Item = u7>>(&mut self, data: I) -> &mut Self;
    fn replace_payload_range<D, R>(&mut self, data: D, range: R) -> &mut Self
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
        R: core::ops::RangeBounds<usize> + core::iter::Iterator<Item = usize>;
    fn payload<I: core::iter::Iterator<Item = u7>>(&mut self, data: I) -> &mut Self;
}

pub trait SysexBuilderInternal {
    type ByteType;
    fn payload_size(&self) -> usize;
    fn resize(&mut self, payload_size: usize);
    // payload range from the provided start index to the end is moved forward
    // (expanding the buffer) by the provided distance
    fn shift_tail_forward(&mut self, payload_index_tail_start: usize, distance: usize);
    // payload range from the provided start index to the end is moved backward
    // (contracting the buffer) by the provided distance
    fn shift_tail_backward(&mut self, payload_index_tail_start: usize, distance: usize);
    // write the payload data into the buffer starting from the
    // provided start index.
    // NOTE: the caller must ensure there is enough space in the buffer and
    // that they wont overwrite any important data.
    fn write_payload<D: core::iter::Iterator<Item = Self::ByteType>>(
        &mut self,
        payload_index_tail_start: usize,
        data: D,
    ) {
        for (d, i) in data.zip(payload_index_tail_start..) {
            self.write_datum(d, i);
        }
    }
    // write the payload datum into the buffer starting at the
    // provided index.
    // NOTE: the caller must ensure there is enough space in the buffer and
    // that they wont overwrite any important data.
    fn write_datum(&mut self, datum: Self::ByteType, payload_index: usize);
}

pub trait Streamed: Data {
    fn stream_id(&self) -> u8 {
        self.data()[0].octet(2)
    }
}

pub(crate) trait Level2Message {}
