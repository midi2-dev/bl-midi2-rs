use crate::buffer::{Buffer, BufferMut, Ump};

pub trait Data<B: Buffer> {
    fn data(&self) -> &[B::Unit];
}

pub trait Grouped<B: Ump> {
    fn group(&self) -> crate::u4;
    fn set_group(&mut self, group: crate::u4)
    where
        B: BufferMut;
}

pub trait Channeled<B: Buffer> {
    fn channel(&self) -> crate::u4;
    fn set_channel(&mut self, channel: crate::u4)
    where
        B: BufferMut;
}

pub trait Streamed<B: Ump> {
    fn stream_id(&self) -> u8;
    fn set_stream_id(&mut self, stream_id: u8)
    where
        B: BufferMut;
}

pub trait RebufferFrom<T>: Sized {
    fn rebuffer_from(value: T) -> Self;
}

pub trait RebufferInto<T>: Sized {
    fn rebuffer_into(self) -> T;
}

impl<T, V> RebufferInto<V> for T
where
    V: RebufferFrom<T>,
{
    fn rebuffer_into(self) -> V {
        <V as RebufferFrom<T>>::rebuffer_from(self)
    }
}

pub trait TryRebufferFrom<T>: Sized {
    fn try_rebuffer_from(value: T) -> core::result::Result<Self, crate::error::BufferOverflow>;
}

pub trait TryRebufferInto<T>: Sized {
    fn try_rebuffer_into(self) -> core::result::Result<T, crate::error::BufferOverflow>;
}

impl<T, V> TryRebufferInto<V> for T
where
    V: TryRebufferFrom<T>,
{
    fn try_rebuffer_into(self) -> core::result::Result<V, crate::error::BufferOverflow> {
        <V as TryRebufferFrom<T>>::try_rebuffer_from(self)
    }
}

pub trait FromBytes<T>: Sized {
    fn from_bytes(other: T) -> Self;
}

pub trait IntoUmp<T> {
    fn into_ump(self) -> T;
}

impl<T, U> IntoUmp<U> for T
where
    U: FromBytes<T>,
{
    fn into_ump(self) -> U {
        <U as FromBytes<T>>::from_bytes(self)
    }
}

pub trait FromUmp<T>: Sized {
    fn from_ump(other: T) -> Self;
}

pub trait IntoBytes<T> {
    fn into_bytes(self) -> T;
}

impl<T, U> IntoBytes<U> for T
where
    U: FromUmp<T>,
{
    fn into_bytes(self) -> U {
        <U as FromUmp<T>>::from_ump(self)
    }
}

pub trait TryFromBytes<T>: Sized {
    fn try_from_bytes(other: T) -> Result<Self, crate::error::BufferOverflow>;
}

pub trait TryIntoUmp<T> {
    fn try_into_ump(self) -> Result<T, crate::error::BufferOverflow>;
}

impl<T, U> TryIntoUmp<U> for T
where
    U: TryFromBytes<T>,
{
    fn try_into_ump(self) -> Result<U, crate::error::BufferOverflow> {
        <U as TryFromBytes<T>>::try_from_bytes(self)
    }
}

pub trait TryFromUmp<T>: Sized {
    fn try_from_ump(other: T) -> Result<Self, crate::error::BufferOverflow>;
}

pub trait TryIntoBytes<T> {
    fn try_into_bytes(self) -> Result<T, crate::error::BufferOverflow>;
}

impl<T, U> TryIntoBytes<U> for T
where
    U: TryFromUmp<T>,
{
    fn try_into_bytes(self) -> Result<U, crate::error::BufferOverflow> {
        <U as TryFromUmp<T>>::try_from_ump(self)
    }
}

pub trait Sysex<B: crate::buffer::Buffer> {
    type Byte;
    type PayloadIterator: core::iter::Iterator<Item = Self::Byte>;

    fn payload(&self) -> Self::PayloadIterator;
    fn set_payload<D>(&mut self, data: D) -> crate::result::Result<()>
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut;
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

pub(crate) trait MinSize<B: Buffer> {
    fn min_size() -> usize;
}

pub(crate) trait Size<B: Buffer> {
    fn size(&self) -> usize;
}
