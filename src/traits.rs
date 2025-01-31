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
/// let owned = NoteOn::<[u32; 4]>::rebuffer_from(borrowed);
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
/// let owned: NoteOn<[u32; 4]> = borrowed.rebuffer_into();
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
