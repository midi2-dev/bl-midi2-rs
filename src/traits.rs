pub trait Data<B: crate::buffer::Buffer> {
    fn data(&self) -> &[B::Unit];
}

pub trait Grouped<B: crate::buffer::Ump>: Data<B> {
    fn group(&self) -> crate::u4;
    fn set_group(&mut self, group: crate::u4)
    where
        B: crate::buffer::BufferMut;
}

pub trait Channeled<B: crate::buffer::Buffer> {
    fn channel(&self) -> crate::u4;
    fn set_channel(&mut self, channel: crate::u4)
    where
        B: crate::buffer::BufferMut;
}

pub trait Streamed<B: crate::buffer::Ump> {
    fn stream_id(&self) -> u8;
    fn set_stream_id(&mut self, channel: u8)
    where
        B: crate::buffer::BufferMut;
}

pub trait Sysex {
    type ByteType;
    type PayloadIterator: core::iter::Iterator<Item = u8>;

    fn payload(&self) -> Self::PayloadIterator;
    fn append_payload<D>(&mut self, data: D) -> crate::result::Result<()>
    where
        D: core::iter::Iterator<Item = Self::ByteType>;
    fn insert_payload<D>(&mut self, data: D, before: usize) -> crate::result::Result<()>
    where
        D: core::iter::Iterator<Item = Self::ByteType>;
    fn replace_payload_range<D, R>(&mut self, data: D, range: R) -> crate::result::Result<()>
    where
        D: core::iter::Iterator<Item = Self::ByteType>,
        R: core::ops::RangeBounds<usize> + core::iter::Iterator<Item = usize>;
    fn set_payload<D>(&mut self, data: D) -> crate::result::Result<()>
    where
        D: core::iter::Iterator<Item = Self::ByteType>;
}

pub(crate) trait SysexInternal {
    type ByteType;
    fn payload_size(&self) -> usize;
    fn resize(&mut self, payload_size: usize);
    // payload range from the provided start index to the end is moved forward
    // (expanding the buffer) by the provided distance
    fn shift_tail_forward(&mut self, payload_index_tail_start: usize, distance: usize);
    // payload range from the provided start index to the end is moved backward
    // (contracting the buffer) by the provided distance
    fn shift_tail_backward(&mut self, payload_index_tail_start: usize, distance: usize);
    // write the payload datum into the buffer starting at the
    // provided index.
    // NOTE: the caller must ensure there is enough space in the buffer and
    // that they won't overwrite any important data.
    fn write_datum(&mut self, datum: Self::ByteType, payload_index: usize);
}

pub(crate) trait Level2Message {}
