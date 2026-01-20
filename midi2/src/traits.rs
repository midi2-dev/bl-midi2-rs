use crate::buffer::{Buffer, BufferMut, Ump};

/// View the wrapped data of the MIDI message as a slice of units.
///
/// A slice of [u32] for [Ump] backed messages.
///
/// ```rust
/// use midi2::{Data, channel_voice1::NoteOn};
///
/// let message = NoteOn::<[u32; 4]>::new();
///
/// assert_eq!(message.data(), &[0x2090_0000]);
/// ```
///
/// A slice of [u8] for [Bytes](crate::buffer::Bytes) backed messages.
///
/// ```rust
/// use midi2::{Data, channel_voice1::NoteOn};
///
/// let message = NoteOn::<[u8; 3]>::new();
///
/// assert_eq!(message.data(), &[0x90, 0x00, 0x00]);
/// ```
pub trait Data<B: Buffer> {
    fn data(&self) -> &[B::Unit];
}

/// Read and write the MIDI 2.0 group field of a wrapped MIDI message.
///
/// ```rust
/// use midi2::{ux::u4, Grouped, Data, channel_voice2::NoteOn};
///
/// let mut message = NoteOn::<[u32; 4]>::new();
///
/// message.set_group(u4::new(0xA));
///
/// assert_eq!(message.group(), u4::new(0xA));
/// assert_eq!(message.data(), &[0x4A90_0000, 0x0000_0000]);
/// ```
pub trait Grouped<B: Ump> {
    fn group(&self) -> crate::ux::u4;
    fn set_group(&mut self, group: crate::ux::u4)
    where
        B: BufferMut;
}

/// Read and write the channel field of a wrapped MIDI message.
///
/// ```rust
/// use midi2::{ux::u4, Channeled, Data, channel_voice2::NoteOn};
///
/// let mut message = NoteOn::<[u32; 4]>::new();
///
/// message.set_channel(u4::new(0x5));
///
/// assert_eq!(message.channel(), u4::new(0x5));
/// assert_eq!(message.data(), &[0x4095_0000, 0x0000_0000]);
/// ```
pub trait Channeled<B: Buffer> {
    fn channel(&self) -> crate::ux::u4;
    fn set_channel(&mut self, channel: crate::ux::u4)
    where
        B: BufferMut;
}

/// Convert a generic message from one [buffer](crate::buffer) specialisation to another.
///
/// ```rust
/// use midi2::{RebufferFrom, Data, channel_voice1::NoteOn};
///
/// let borrowed: NoteOn<&[u32]> = NoteOn::try_from(&[0x2D9E_753D_u32][..]).expect("Valid data");
/// let owned = NoteOn::<std::vec::Vec<u32>>::rebuffer_from(borrowed);
///
/// assert_eq!(owned.data(), &[0x2D9E_753D]);
/// ```
pub trait RebufferFrom<T>: Sized {
    fn rebuffer_from(value: T) -> Self;
}

/// Convert a generic message into a different [buffer](crate::buffer) specialisation.
///
/// ```rust
/// use midi2::{RebufferInto, Data, channel_voice1::NoteOn};
///
/// let borrowed: NoteOn<&[u32]> = NoteOn::try_from(&[0x2D9E_753D_u32][..]).expect("Valid data");
/// let owned: NoteOn<std::vec::Vec<u32>> = borrowed.rebuffer_into();
///
/// assert_eq!(owned.data(), &[0x2D9E_753D]);
/// ```
///
/// Note that this trait has a blanket implementation for all messages which implement
/// [RebufferFrom] (similar to the standard [core::convert::Into] trait)
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

/// Convert from a generic message to an array-backed one.
///
/// Implementers this trait do some compile-time magic to ensure
/// that the target array buffer is large enough to fit the min size of the
/// fixed size message.
///
/// ```rust
/// use midi2::{ArrayRebufferFrom, Data, channel_voice1::NoteOn};
///
/// let borrowed: NoteOn<&[u32]> = NoteOn::try_from(&[0x2D9E_753D_u32][..]).expect("Valid data");
/// let owned = NoteOn::<[u32; 4]>::array_rebuffer_from(borrowed);
///
/// assert_eq!(owned.data(), &[0x2D9E_753D]);
/// ```
pub trait ArrayRebufferFrom<T>: Sized {
    fn array_rebuffer_from(value: T) -> Self;
}

/// Convert a generic message into an array-backed specialisation.
///
/// Implementers this trait do some compile-time magic to ensure
/// that the target array buffer is large enough to fit the min size of the
/// fixed size message.
///
/// ```rust
/// use midi2::{ArrayRebufferInto, Data, channel_voice1::NoteOn};
///
/// let borrowed: NoteOn<&[u32]> = NoteOn::try_from(&[0x2D9E_753D_u32][..]).expect("Valid data");
/// let owned: NoteOn<[u32; 4]> = borrowed.array_rebuffer_into();
///
/// assert_eq!(owned.data(), &[0x2D9E_753D]);
/// ```
///
/// Note that this trait has a blanket implementation for all messages which implement
/// [RebufferFrom] (similar to the standard [core::convert::Into] trait)
pub trait ArrayRebufferInto<T>: Sized {
    fn array_rebuffer_into(self) -> T;
}
impl<T, V> ArrayRebufferInto<V> for T
where
    V: ArrayRebufferFrom<T>,
{
    fn array_rebuffer_into(self) -> V {
        <V as ArrayRebufferFrom<T>>::array_rebuffer_from(self)
    }
}

/// Attempt to convert a generic message from one [buffer](crate::buffer) specialisation to another.
///
/// The conversion may fail with a [BufferOverflow](crate::error::BufferOverflow) error if the
/// target message representation does not fit all of the message data.
///
/// ```rust
/// use midi2::{TryRebufferFrom, sysex7::Sysex7};
///
/// let borrowed: Sysex7<&[u32]> = Sysex7::try_from(&[
///     0x3016_0001,
///     0x0203_0405,
///     0x3035_0607,
///     0x0809_0A00,
/// ][..]).expect("Valid data");
///
/// assert!(Sysex7::<[u32; 4]>::try_rebuffer_from(borrowed.clone()).is_ok());
/// assert!(Sysex7::<[u32; 2]>::try_rebuffer_from(borrowed.clone()).is_err());
/// ```
pub trait TryRebufferFrom<T>: Sized {
    fn try_rebuffer_from(value: T) -> core::result::Result<Self, crate::error::BufferOverflow>;
}

/// Attempt to convert a generic message into a different [buffer](crate::buffer) specialisation.
///
/// The conversion may fail with a [BufferOverflow](crate::error::BufferOverflow) error if the
/// target message representation does not fit all of the message data.
///
/// ```rust
/// use midi2::{TryRebufferInto, sysex7::Sysex7, error::BufferOverflow};
///
/// let borrowed: Sysex7<&[u32]> = Sysex7::try_from(&[
///     0x3016_0001,
///     0x0203_0405,
///     0x3035_0607,
///     0x0809_0A00,
/// ][..]).expect("Valid data");
///
/// let arr4: Result<Sysex7<[u32; 4]>, BufferOverflow>  = borrowed
///     .clone()
///     .try_rebuffer_into();
/// arr4.expect("Buffer is large enough");
///
/// let arr2: Result<Sysex7<[u32; 2]>, BufferOverflow> = borrowed
///     .clone()
///     .try_rebuffer_into();
/// arr2.expect_err("Buffer is too small");
/// ```
/// Note that this trait has a blanket implementation for all messages which implement
/// [TryRebufferFrom] (similar to the standard [core::convert::TryInto] trait)
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

/// Convert a message from a [Bytes](crate::buffer::Bytes) backing buffer
/// to a [Ump] backing buffer.
///
/// ```rust
/// use midi2::{FromBytes, Data, channel_voice1::NoteOn};
///
/// let bytes_message = NoteOn::try_from(&[0x9E_u8, 0x75, 0x3D][..]).expect("Valid data");
/// let ump_message = NoteOn::<[u32; 4]>::from_bytes(bytes_message);
///
/// assert_eq!(ump_message.data(), &[0x209E_753D]);
/// ```
pub trait FromBytes<T>: Sized {
    fn from_bytes(other: T) -> Self;
}

/// Convert a [Bytes](crate::buffer::Bytes) backed message into a [Ump] backed message.
///
/// ```rust
/// use midi2::{IntoUmp, Data, channel_voice1::NoteOn};
///
/// let bytes_message = NoteOn::try_from(&[0x9E_u8, 0x75, 0x3D][..]).expect("Valid data");
/// let ump_message: NoteOn<[u32; 4]> = bytes_message.into_ump();
///
/// assert_eq!(ump_message.data(), &[0x209E_753D]);
/// ```
///
/// Note that this is the reciprocal trait to [FromBytes].
/// Any implementer of [FromBytes] automatically implements [IntoUmp],
/// similar to the [core::convert::Into] trait.
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

/// Convert a message from a [Ump] backing buffer
/// to a [Bytes](crate::buffer::Bytes) backing buffer.
///
/// Note that in most cases this is a "lossy" conversion. Some
/// fields in a [Ump] message are not represented in a [Bytes](crate::buffer::Bytes)
/// message like `group`, for example.
///
/// ```rust
/// use midi2::{FromUmp, Data, channel_voice1::NoteOn};
///
/// let ump_message = NoteOn::try_from(&[0x279E_753D_u32][..]).expect("Valid data");
/// let bytes_message = NoteOn::<[u8; 3]>::from_ump(ump_message);
///
/// assert_eq!(bytes_message.data(), &[0x9E, 0x75, 0x3D]);
/// ```
pub trait FromUmp<T>: Sized {
    fn from_ump(other: T) -> Self;
}

/// Convert a [Ump] backed message into a [Bytes](crate::buffer::Bytes) backed message.
///
/// Note that in most cases this is a "lossy" conversion. Some
/// fields in a [Ump] message are not represented in a [Bytes](crate::buffer::Bytes)
/// message like `group`, for example.
///
/// ```rust
/// use midi2::{IntoBytes, Data, channel_voice1::NoteOn};
///
/// let ump_message = NoteOn::try_from(&[0x279E_753D_u32][..]).expect("Valid data");
/// let bytes_message: NoteOn<[u8; 3]> = ump_message.into_bytes();
///
/// assert_eq!(bytes_message.data(), &[0x9E, 0x75, 0x3D]);
/// ```
/// This is the reciprocal trait to [FromUmp].
/// Any implementer of [FromUmp] automatically implements [IntoBytes],
/// similar to the [core::convert::Into] trait.
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

/// Attempt to convert a message from a [Bytes](crate::buffer::Bytes) backing buffer
/// to a [Ump] backing buffer.
///
/// The conversion may fail with a [BufferOverflow](crate::error::BufferOverflow) error
/// if the target buffer is not large enough to contain the data.
///
/// ```rust
/// use midi2::{TryFromBytes, Data, sysex7::Sysex7};
///
/// let bytes_message = Sysex7::try_from(&[
///     0xF0_u8, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xF7,
/// ][..]).expect("Valid data");
///
/// let ump_message = Sysex7::<[u32; 4]>::try_from_bytes(bytes_message.clone());
///
/// assert_eq!(ump_message.expect("Buffer is large enough").data(), &[
///     0x3016_0001,
///     0x0203_0405,
///     0x3034_0607,
///     0x0809_0000,
/// ]);
///
/// let small_ump_message = Sysex7::<[u32; 2]>::try_from_bytes(bytes_message.clone());
/// small_ump_message.expect_err("Buffer is too small");
/// ```
pub trait TryFromBytes<T>: Sized {
    fn try_from_bytes(other: T) -> Result<Self, crate::error::BufferOverflow>;
}

/// Attempt to convert a [Bytes](crate::buffer::Bytes) backed message into a [Ump] backed message.
///
/// The conversion may fail with a [BufferOverflow](crate::error::BufferOverflow) error
/// if the target buffer is not large enough to contain the data.
///
/// ```rust
/// use midi2::{TryIntoUmp, Data, sysex7::Sysex7, error::BufferOverflow};
///
/// let bytes_message = Sysex7::try_from(&[
///     0xF0_u8, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xF7,
/// ][..]).expect("Valid data");
///
/// let ump_message: Result<Sysex7<[u32; 4]>, BufferOverflow> = bytes_message
///     .clone()
///     .try_into_ump();
///
/// assert_eq!(ump_message.expect("Buffer is large enough").data(), &[
///     0x3016_0001,
///     0x0203_0405,
///     0x3034_0607,
///     0x0809_0000,
/// ]);
///
/// let small_ump_message: Result<Sysex7<[u32; 2]>, BufferOverflow> = bytes_message
///     .clone()
///     .try_into_ump();
/// small_ump_message.expect_err("Buffer is too small");
/// ```
///
/// Note that this is the reciprocal trait to [TryFromBytes].
/// Any implementer of [TryFromBytes] automatically implements [IntoUmp],
/// similar to the [core::convert::TryInto] trait.
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

/// Attempt to convert a message from a [Ump] backing buffer
/// to a [Bytes](crate::buffer::Bytes) backing buffer.
///
/// The conversion may fail with a [BufferOverflow](crate::error::BufferOverflow) error
/// if the target buffer is not large enough to contain the data.
///
/// Note that in most cases this is a "lossy" conversion. Some
/// fields in a [Ump] message are not represented in a [Bytes](crate::buffer::Bytes)
/// message like `group`, for example.
///
/// ```rust
/// use midi2::{TryFromUmp, Data, sysex7::Sysex7};
///
/// let ump_message = Sysex7::try_from(&[
///     0x3016_0001_u32,
///     0x0203_0405,
///     0x3034_0607,
///     0x0809_0000,
/// ][..]).expect("Valid data");
///
/// let bytes_message = Sysex7::<[u8; 12]>::try_from_ump(ump_message.clone());
///
/// assert_eq!(bytes_message.expect("Buffer is large enough").data(), &[
///     0xF0, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xF7,
/// ]);
///
/// let small_bytes_message = Sysex7::<[u8; 10]>::try_from_ump(ump_message.clone());
/// small_bytes_message.expect_err("Buffer is too small");
/// ```
pub trait TryFromUmp<T>: Sized {
    fn try_from_ump(other: T) -> Result<Self, crate::error::BufferOverflow>;
}

/// Attempt to convert a [Ump] backed message into a [Bytes](crate::buffer::Bytes) backed message.
///
/// The conversion may fail with a [BufferOverflow](crate::error::BufferOverflow) error
/// if the target buffer is not large enough to contain the data.
///
/// Note that in most cases this is a "lossy" conversion. Some
/// fields in a [Ump] message are not represented in a [Bytes](crate::buffer::Bytes)
/// message like `group`, for example.
///
/// ```rust
/// use midi2::{TryIntoBytes, Data, sysex7::Sysex7, error::BufferOverflow};
///
/// let ump_message = Sysex7::try_from(&[
///     0x3016_0001_u32,
///     0x0203_0405,
///     0x3034_0607,
///     0x0809_0000,
/// ][..]).expect("Valid data");
///
/// let bytes_message: Result<Sysex7<[u8; 12]>, BufferOverflow> = ump_message
///     .clone()
///     .try_into_bytes();
///
/// assert_eq!(bytes_message.expect("Buffer is large enough").data(), &[
///     0xF0, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xF7,
/// ]);
///
/// let small_bytes_message: Result<Sysex7<[u8; 10]>, BufferOverflow> = ump_message
///     .try_into_bytes()
///     .clone();
/// small_bytes_message.expect_err("Buffer is too small");
/// ```
///
/// This is the reciprocal trait to [TryFromUmp].
/// Any implementer of [TryFromUmp] automatically implements [TryIntoBytes],
/// similar to the [core::convert::TryInto] trait.
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

/// Read and write the payload data on a MIDI sysex message.
///
/// Payload data can be read as an iterator over the [Sysex::Byte] type.
///
/// ```rust
/// use midi2::{sysex8::Sysex8, Sysex};
///
/// let message = Sysex8::try_from(&[
///     0x501E_0000,
///     0x0102_0304,
///     0x0506_0708,
///     0x090A_0B0C,
///     0x5038_000D,
///     0x0E0F_1011,
///     0x1213_0000,
///     0x0000_0000,
/// ][..]).expect("Valid data");
///
/// assert_eq!(message.payload().collect::<Vec<u8>>(), (0..20).collect::<Vec<u8>>());
/// ```
///
/// When the backing buffer implements [BufferResize](crate::buffer::BufferResize)
/// payload data can be set using [set_payload](Sysex::set_payload).
///
/// ```rust
/// use midi2::{sysex8::Sysex8, Sysex, Data};
///
/// let mut message = Sysex8::<Vec<u32>>::new();
/// message.set_payload(0..20);
///
/// assert_eq!(message.data(), &[
///     0x501E_0000,
///     0x0102_0304,
///     0x0506_0708,
///     0x090A_0B0C,
///     0x5038_000D,
///     0x0E0F_1011,
///     0x1213_0000,
///     0x0000_0000,
/// ]);
/// ```
///
/// When the backing buffer implements [BufferTryResize](crate::buffer::BufferTryResize)
/// payload data can be set using [try_set_payload](Sysex::try_set_payload).
///
/// ```rust
/// use midi2::{sysex8::Sysex8, Sysex, Data};
///
/// let mut message = Sysex8::<[u32; 8]>::new();
///
/// assert!(message.try_set_payload(0..20).is_ok());
/// assert_eq!(message.data(), &[
///     0x501E_0000,
///     0x0102_0304,
///     0x0506_0708,
///     0x090A_0B0C,
///     0x5038_000D,
///     0x0E0F_1011,
///     0x1213_0000,
///     0x0000_0000,
/// ]);
///
/// assert!(message.try_set_payload(0..30).is_err());
/// ```
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
    fn payload_size(&self) -> usize;
    /// Sets the value of the byte at position `index` in the payload.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to `payload_size`
    fn set_byte(&mut self, byte: Self::Byte, index: usize)
    where
        B: crate::buffer::BufferMut;
    /// Insert the provided byte data before position `index`
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than `payload_size`
    fn insert_payload<D>(&mut self, data: D, index: usize)
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferResize,
    {
        self.splice_payload(data, index..index)
    }

    /// Insert the provided byte data before position `index`
    ///
    /// # Fails
    ///
    /// If the underlying buffer cannot resize to accommodate the new data.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than `payload_size`
    fn try_insert_payload<D>(
        &mut self,
        data: D,
        index: usize,
    ) -> core::result::Result<(), crate::error::BufferOverflow>
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    {
        self.try_splice_payload(data, index..index)
    }

    /// Pushes the provided payload data iterator into the back of the
    /// existing message payload.
    fn append_payload<D>(&mut self, data: D)
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferResize,
    {
        self.insert_payload(data, self.payload_size());
    }

    /// Pushes the provided payload data iterator into the back of the
    /// existing message payload.
    ///
    /// # Fails
    ///
    /// When the underlying buffer cannot resize to accommodate the new data.
    fn try_append_payload<D>(
        &mut self,
        data: D,
    ) -> core::result::Result<(), crate::error::BufferOverflow>
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    {
        self.try_insert_payload(data, self.payload_size())
    }

    /// Replaces the specified payload range with the given `data` iterator.
    /// `data` does not need to be the same length as range.
    fn splice_payload<D, R>(&mut self, data: D, range: R)
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferResize,
        R: core::ops::RangeBounds<usize>;

    /// Attempt to replace the specified payload range with the given `data` iterator.
    /// `data` does not need to be the same length as range.
    /// Fails if the underlying buffer cannot resize to accommodate the new data.
    fn try_splice_payload<D, R>(
        &mut self,
        data: D,
        range: R,
    ) -> core::result::Result<(), crate::error::BufferOverflow>
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize,
        R: core::ops::RangeBounds<usize>;

    /// Pushes the provided byte into the back of the
    /// existing message payload.
    fn append_byte(&mut self, byte: Self::Byte)
    where
        B: crate::buffer::BufferMut + crate::buffer::BufferResize,
    {
        self.insert_payload(core::iter::once(byte), self.payload_size());
    }

    /// Pushes the provided byte into the back of the
    /// existing message payload.
    ///
    /// # Fails
    ///
    /// When the underlying buffer cannot resize to accommodate the new data.
    fn try_append_byte(
        &mut self,
        byte: Self::Byte,
    ) -> core::result::Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    {
        self.try_insert_payload(core::iter::once(byte), self.payload_size())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SysexTryResizeError(pub usize);

pub(crate) trait SysexInternal<B: crate::buffer::Buffer>: Sysex<B> {
    // Ensure that the payload optimally fills the underlying buffer.
    fn compact(&mut self)
    where
        B: crate::buffer::BufferMut;

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

    // Moves the payload in-place such that the bytes [start, len) move to
    // position `to`.
    // This can be used to move the tail forward or backward.
    // Note that this function does not change the size of the payload,
    // nor the size of the underlying buffer.
    // If the tail is moved towards the end of the payload then it overwrites
    // the bytes at the end. Similarlarly when the tail is move towards the
    // front, it overwrites the bytes at the start. It will also not zero
    // the copied-from bytes - these will be left 'dirty'.
    fn move_payload_tail(&mut self, tail: usize, to: usize)
    where
        B: crate::buffer::BufferMut;

    // write byte into the buffer at the provided index.
    // NOTE: the caller must ensure the buffer is large enough
    fn write_datum(&mut self, datum: Self::Byte, payload_index: usize)
    where
        B: crate::buffer::BufferMut;
}

pub(crate) trait MinSize<B: Buffer> {
    const MIN_SIZE: usize;
}

pub(crate) trait ArraySizeValid<const SIZE: usize, B: Buffer>: MinSize<B> {
    const VALID: ();
}

impl<const SIZE: usize, B: Buffer, M: MinSize<B>> ArraySizeValid<SIZE, B> for M {
    const VALID: () = if SIZE < <Self as MinSize<B>>::MIN_SIZE {
        panic!("Array is shorter than minimum message size");
    };
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

mod conversion {
    pub(crate) trait Center:
        Into<u32> + TryFrom<u32> + Sized + Copy + PartialEq + PartialOrd
    where
        <Self as TryFrom<u32>>::Error: core::fmt::Debug,
    {
        fn min_value() -> Self;
        fn max_value() -> Self;

        fn center_value() -> Self {
            let min: u32 = Self::min_value().into();
            let max: u32 = Self::max_value().into();

            ((max - min) / 2_u32)
                .try_into()
                .expect("Center shouldn't be larger than max.")
        }
    }

    impl Center for ux::u7 {
        fn min_value() -> Self {
            Self::min_value()
        }
        fn max_value() -> Self {
            Self::max_value()
        }
    }

    impl Center for ux::u9 {
        fn min_value() -> Self {
            Self::min_value()
        }
        fn max_value() -> Self {
            Self::max_value()
        }
    }

    impl Center for ux::u14 {
        fn min_value() -> Self {
            Self::min_value()
        }
        fn max_value() -> Self {
            Self::max_value()
        }
    }

    impl Center for u16 {
        fn min_value() -> Self {
            Self::min_value()
        }
        fn max_value() -> Self {
            Self::max_value()
        }
    }

    impl Center for u32 {
        fn min_value() -> Self {
            Self::min_value()
        }
        fn max_value() -> Self {
            Self::max_value()
        }
    }

    pub(crate) trait MinCenterMax: Center + core::ops::Shr<u32>
    where
        <Self as core::ops::Shr<u32>>::Output: Into<Self>,
        <Self as TryFrom<u32>>::Error: core::fmt::Debug,
    {
        fn upscale<U: Center>(self) -> U
        where
            Self: Into<U>,
            U: core::ops::Shl<u32>,
            U: From<<U as core::ops::Shl<u32>>::Output>,
            <U as TryFrom<u32>>::Error: core::fmt::Debug,
        {
            let min = Self::min_value();
            let center = Self::center_value();
            let max = Self::max_value();

            match self {
                s if s == min => U::min_value(),
                s if s == max => U::max_value(),
                s if s == center => U::center_value(),
                s if (min..center).contains(&s) => {
                    let self_max: u32 = Self::max_value().into();
                    let other_max: u32 = U::max_value().into();
                    let shift = (other_max - self_max).count_ones();
                    let other: U = self.into();
                    let upscaled = other << shift;
                    upscaled.into()
                }
                s if (center..max).contains(&s) => s.into(),
                _ => self.into(),
            }
        }

        fn downscale<U: Center + TryFrom<Self>>(self) -> U
        where
            <U as TryFrom<u32>>::Error: core::fmt::Debug,
            <U as TryFrom<Self>>::Error: core::fmt::Debug,
        {
            let self_max: u32 = Self::max_value().into();
            let other_max: u32 = U::max_value().into();
            let shift = (self_max - other_max).count_ones();
            let downscaled = self >> shift;
            let downscaled: Self = downscaled.into();

            downscaled.try_into().expect("Downscaling should not fail.")
        }
    }

    impl<U: Center + core::ops::Shr<u32>> MinCenterMax for U
    where
        <U as TryFrom<u32>>::Error: core::fmt::Debug,
        <U as core::ops::Shr<u32>>::Output: Into<U>,
    {
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_mins_upscaling() {
            let min_u7 = ux::u7::new(0);
            let min_u9 = ux::u9::new(0);
            let min_u14 = ux::u14::new(0);
            let min_u16 = 0_u16;
            let min_u32 = 0_u32;

            assert_eq!(min_u7.upscale::<ux::u7>(), min_u7);
            assert_eq!(min_u7.upscale::<ux::u9>(), min_u9);
            assert_eq!(min_u7.upscale::<ux::u14>(), min_u14);
            assert_eq!(min_u7.upscale::<u16>(), min_u16);
            assert_eq!(min_u7.upscale::<u32>(), min_u32);

            assert_eq!(min_u9.upscale::<ux::u9>(), min_u9);
            assert_eq!(min_u9.upscale::<ux::u14>(), min_u14);
            assert_eq!(min_u9.upscale::<u16>(), min_u16);
            assert_eq!(min_u9.upscale::<u32>(), min_u32);

            assert_eq!(min_u14.upscale::<ux::u14>(), min_u14);
            assert_eq!(min_u14.upscale::<u16>(), min_u16);
            assert_eq!(min_u14.upscale::<u32>(), min_u32);

            assert_eq!(min_u16.upscale::<u16>(), min_u16);
            assert_eq!(min_u16.upscale::<u32>(), min_u32);

            assert_eq!(min_u32.upscale::<u32>(), min_u32);
        }

        #[test]
        fn test_maxs_upscaling() {
            let max_u7 = ux::u7::new(127);
            let max_u9 = ux::u9::new(511);
            let max_u14 = ux::u14::new(16383);
            let max_u16 = 0xFFFF_u16;
            let max_u32 = 0xFFFFFFFF_u32;

            assert_eq!(max_u7.upscale::<ux::u7>(), max_u7);
            assert_eq!(max_u7.upscale::<ux::u9>(), max_u9);
            assert_eq!(max_u7.upscale::<ux::u14>(), max_u14);
            assert_eq!(max_u7.upscale::<u16>(), max_u16);
            assert_eq!(max_u7.upscale::<u32>(), max_u32);

            assert_eq!(max_u9.upscale::<ux::u9>(), max_u9);
            assert_eq!(max_u9.upscale::<ux::u14>(), max_u14);
            assert_eq!(max_u9.upscale::<u16>(), max_u16);
            assert_eq!(max_u9.upscale::<u32>(), max_u32);

            assert_eq!(max_u14.upscale::<ux::u14>(), max_u14);
            assert_eq!(max_u14.upscale::<u16>(), max_u16);
            assert_eq!(max_u14.upscale::<u32>(), max_u32);

            assert_eq!(max_u16.upscale::<u16>(), max_u16);
            assert_eq!(max_u16.upscale::<u32>(), max_u32);

            assert_eq!(max_u32.upscale::<u32>(), max_u32);
        }

        #[test]
        fn test_centers_upscaling() {
            let center_u7 = ux::u7::new(63);
            let center_u9 = ux::u9::new(255);
            let center_u14 = ux::u14::new(8191);
            let center_u16 = 0x7FFF_u16;
            let center_u32 = 0x7FFFFFFF_u32;

            assert_eq!(center_u7.upscale::<ux::u7>(), center_u7);
            assert_eq!(center_u7.upscale::<ux::u9>(), center_u9);
            assert_eq!(center_u7.upscale::<ux::u14>(), center_u14);
            assert_eq!(center_u7.upscale::<u16>(), center_u16);
            assert_eq!(center_u7.upscale::<u32>(), center_u32);

            assert_eq!(center_u9.upscale::<ux::u9>(), center_u9);
            assert_eq!(center_u9.upscale::<ux::u14>(), center_u14);
            assert_eq!(center_u9.upscale::<u16>(), center_u16);
            assert_eq!(center_u9.upscale::<u32>(), center_u32);

            assert_eq!(center_u14.upscale::<ux::u14>(), center_u14);
            assert_eq!(center_u14.upscale::<u16>(), center_u16);
            assert_eq!(center_u14.upscale::<u32>(), center_u32);

            assert_eq!(center_u16.upscale::<u16>(), center_u16);
            assert_eq!(center_u16.upscale::<u32>(), center_u32);

            assert_eq!(center_u32.upscale::<u32>(), center_u32);
        }

        #[test]
        fn test_lower_range_downscaling() {
            let lower_u7 = ux::u7::new(31);
            let lower_u9 = ux::u9::new(127);
            let lower_u14 = ux::u14::new(4095);
            let lower_u16 = 0x3FFF_u16;
            let lower_u32 = 0x3FFFFFFF_u32;

            assert_eq!(lower_u32.downscale::<ux::u7>(), lower_u7);
            assert_eq!(lower_u32.downscale::<ux::u9>(), lower_u9);
            assert_eq!(lower_u32.downscale::<ux::u14>(), lower_u14);
            assert_eq!(lower_u32.downscale::<u16>(), lower_u16);
            assert_eq!(lower_u32.downscale::<u32>(), lower_u32);

            assert_eq!(lower_u16.downscale::<ux::u7>(), lower_u7);
            assert_eq!(lower_u16.downscale::<ux::u9>(), lower_u9);
            assert_eq!(lower_u16.downscale::<ux::u14>(), lower_u14);
            assert_eq!(lower_u16.downscale::<u16>(), lower_u16);

            assert_eq!(lower_u14.downscale::<ux::u7>(), lower_u7);
            assert_eq!(lower_u14.downscale::<ux::u9>(), lower_u9);
            assert_eq!(lower_u14.downscale::<ux::u14>(), lower_u14);

            assert_eq!(lower_u9.downscale::<ux::u7>(), lower_u7);
            assert_eq!(lower_u9.downscale::<ux::u9>(), lower_u9);

            assert_eq!(lower_u7.downscale::<ux::u7>(), lower_u7);
        }

        #[test]
        fn test_lower_range_upscaling() {
            let lower_u7 = ux::u7::new(0x1F);
            let lower_u9 = ux::u9::new(0x7C);
            let lower_u14 = ux::u14::new(0xF80);
            let lower_u16 = 0x3E00_u16;
            let lower_u32 = 0x3E00_0000_u32;

            assert_eq!(lower_u7.upscale::<ux::u7>(), lower_u7);
            assert_eq!(lower_u7.upscale::<ux::u9>(), lower_u9);
            assert_eq!(lower_u7.upscale::<ux::u14>(), lower_u14);
            assert_eq!(lower_u7.upscale::<u16>(), lower_u16);
            assert_eq!(lower_u7.upscale::<u32>(), lower_u32);

            assert_eq!(lower_u9.upscale::<ux::u9>(), lower_u9);
            assert_eq!(lower_u9.upscale::<ux::u14>(), lower_u14);
            assert_eq!(lower_u9.upscale::<u16>(), lower_u16);
            assert_eq!(lower_u9.upscale::<u32>(), lower_u32);

            assert_eq!(lower_u14.upscale::<ux::u14>(), lower_u14);
            assert_eq!(lower_u14.upscale::<u16>(), lower_u16);
            assert_eq!(lower_u14.upscale::<u32>(), lower_u32);

            assert_eq!(lower_u16.upscale::<u16>(), lower_u16);
            assert_eq!(lower_u16.upscale::<u32>(), lower_u32);

            assert_eq!(lower_u32.upscale::<u32>(), lower_u32);
        }
    }
}
