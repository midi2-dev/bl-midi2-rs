use crate::buffer::{
    Buffer, BufferDefault, BufferFixedSize, BufferMut, BufferResizable, Bytes, Ump, Unit,
};

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
    fn set_stream_id(&mut self, channel: u8)
    where
        B: BufferMut;
}

pub trait RebufferFrom<
    U: Unit,
    A: Buffer<Unit = U>,
    B: Buffer<Unit = U> + BufferMut + BufferDefault + BufferResizable,
    T,
>: Sized
{
    fn rebuffer_from(value: T) -> Self;
}

pub trait RebufferInto<
    U: Unit,
    A: Buffer<Unit = U>,
    B: Buffer<Unit = U> + BufferMut + BufferDefault + BufferResizable,
    T,
>: Sized
{
    fn rebuffer_into(self) -> T;
}

impl<
        U: Unit,
        A: Buffer<Unit = U>,
        B: Buffer<Unit = U> + BufferMut + BufferDefault + BufferResizable,
        T,
        V,
    > RebufferInto<U, A, B, V> for T
where
    V: RebufferFrom<U, A, B, T>,
{
    fn rebuffer_into(self) -> V {
        <V as RebufferFrom<U, A, B, T>>::rebuffer_from(self)
    }
}

impl<
        U: Unit,
        A: Buffer<Unit = U>,
        B: Buffer<Unit = U> + BufferMut + BufferDefault + BufferResizable,
        T,
        V,
    > RebufferFrom<U, A, B, T> for V
where
    V: Data<B> + WithBuffer<B>,
    T: Data<A>,
{
    fn rebuffer_from(value: T) -> Self {
        let mut buffer = <B as BufferDefault>::default();
        buffer.resize(value.data().len());
        buffer.buffer_mut().copy_from_slice(value.data());
        <Self as WithBuffer<B>>::with_buffer(buffer)
    }
}

pub trait TryRebufferFrom<
    U: Unit,
    A: Buffer<Unit = U>,
    B: Buffer<Unit = U> + BufferMut + BufferDefault + BufferFixedSize,
    T,
>: Sized
{
    fn try_rebuffer_from(value: T) -> core::result::Result<Self, crate::error::BufferOverflow>;
}

pub trait TryRebufferInto<
    U: Unit,
    A: Buffer<Unit = U>,
    B: Buffer<Unit = U> + BufferMut + BufferDefault + BufferFixedSize,
    T,
>: Sized
{
    fn try_rebuffer_into(self) -> core::result::Result<T, crate::error::BufferOverflow>;
}

impl<
        U: Unit,
        A: Buffer<Unit = U>,
        B: Buffer<Unit = U> + BufferMut + BufferDefault + BufferFixedSize,
        T,
        V,
    > TryRebufferInto<U, A, B, V> for T
where
    V: TryRebufferFrom<U, A, B, T>,
{
    fn try_rebuffer_into(self) -> core::result::Result<V, crate::error::BufferOverflow> {
        <V as TryRebufferFrom<U, A, B, T>>::try_rebuffer_from(self)
    }
}

impl<
        U: Unit,
        A: Buffer<Unit = U>,
        B: Buffer<Unit = U> + BufferMut + BufferDefault + BufferFixedSize,
        T,
        V,
    > TryRebufferFrom<U, A, B, T> for V
where
    V: Data<B> + WithBuffer<B>,
    T: Data<A>,
{
    fn try_rebuffer_from(value: T) -> core::result::Result<Self, crate::error::BufferOverflow> {
        let mut buffer = <B as BufferDefault>::default();
        if value.data().len() > buffer.buffer().len() {
            return Err(crate::error::BufferOverflow);
        }
        buffer.buffer_mut().copy_from_slice(value.data());
        Ok(<Self as WithBuffer<B>>::with_buffer(buffer))
    }
}

pub trait FromBytes<A: Bytes, B: Ump + BufferMut + BufferDefault + BufferResizable, T> {
    fn from_bytes(other: T) -> Self;
}

pub trait IntoUmp<A: Bytes, B: Ump + BufferMut + BufferDefault + BufferResizable, T> {
    fn into_ump(self) -> T;
}

impl<A: Bytes, B: Ump + BufferMut + BufferDefault + BufferResizable, T, U> IntoUmp<A, B, U> for T
where
    U: FromBytes<A, B, T>,
{
    fn into_ump(self) -> U {
        <U as FromBytes<A, B, T>>::from_bytes(self)
    }
}

pub trait FromUmp<A: Ump, B: Bytes + BufferMut + BufferDefault + BufferResizable, T> {
    fn from_ump(other: T) -> Self;
}

pub trait IntoBytes<A: Ump, B: Bytes + BufferMut + BufferDefault + BufferResizable, T> {
    fn into_bytes(self) -> T;
}

impl<A: Ump, B: Bytes + BufferMut + BufferDefault + BufferResizable, T, U> IntoBytes<A, B, U> for T
where
    U: FromUmp<A, B, T>,
{
    fn into_bytes(self) -> U {
        <U as FromUmp<A, B, T>>::from_ump(self)
    }
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

pub(crate) trait MinSize<B: Buffer> {
    fn min_size() -> usize;
}

pub(crate) trait Size<B: Buffer> {
    fn size(&self) -> usize;
}

pub(crate) trait WithBuffer<B: Buffer> {
    fn with_buffer(buffer: B) -> Self;
}
