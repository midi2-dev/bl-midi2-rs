use crate::buffer::{Buffer, BufferMut, Ump};

pub trait Data<B: Buffer> {
    fn data(&self) -> &[B::Unit];
}

pub trait Grouped<B: Ump> {
    fn group(&self) -> crate::ux::u4;
    fn set_group(&mut self, group: crate::ux::u4)
    where
        B: BufferMut;
}

pub trait Channeled<B: Buffer> {
    fn channel(&self) -> crate::ux::u4;
    fn set_channel(&mut self, channel: crate::ux::u4)
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

pub trait JitterReduced<B: crate::buffer::Ump>: Data<B> {
    fn jitter_reduction(&self) -> Option<crate::utility::JitterReduction>;
    fn set_jitter_reduction(&mut self, jr: Option<crate::utility::JitterReduction>)
    where
        B: crate::buffer::BufferMut;
}

pub trait Sysex<B: crate::buffer::Buffer> {
    type Byte;
    type PayloadIterator<'a>: core::iter::Iterator<Item = Self::Byte>
    where
        B::Unit: 'a,
        Self: 'a;

    fn payload<'a>(&'a self) -> Self::PayloadIterator<'a>
    where
        B::Unit: 'a;
    fn set_payload<D>(&mut self, data: D)
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferResize;
    fn try_set_payload<D>(
        &mut self,
        data: D,
    ) -> core::result::Result<(), crate::error::BufferOverflow>
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SysexTryResizeError(pub usize);

pub(crate) trait SysexInternal<B: crate::buffer::Buffer>: Sysex<B> {
    fn payload_size(&self) -> usize;

    // resize the underlying buffer to accommodate the requested amount
    // of bytes. The newly allocated data should be assumed to be
    // written to immediately after this call - so it doesn't matter
    // if we leave the buffer dirty.
    fn resize(&mut self, payload_size: usize)
    where
        B: crate::buffer::BufferMut + crate::buffer::BufferResize;

    // fallible version of the above
    fn try_resize(&mut self, payload_size: usize) -> core::result::Result<(), SysexTryResizeError>
    where
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize;

    // write byte into the buffer at the provided index.
    // NOTE: the caller must ensure the buffer is large enough
    fn write_datum(&mut self, datum: Self::Byte, payload_index: usize)
    where
        B: crate::buffer::BufferMut;
}

pub(crate) trait MinSize<B: Buffer> {
    fn min_size() -> usize;
}

pub(crate) trait Size<B: Buffer> {
    fn size(&self) -> usize;
}

// Note: not to be used lightly.
// Each use of this is a break of the incapsulation
// of the message type. It's here primarily to allow
// CI messages super powers of their backing sysex7 messages.
pub(crate) trait BufferAccess<B: Buffer> {
    fn buffer_access(&self) -> &B;
    fn buffer_access_mut(&mut self) -> &mut B
    where
        B: BufferMut;
}
