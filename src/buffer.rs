//! Generic backing buffers for messages wrapper types.
//!
//! All messages in midi2 are backed by a generic buffer type.
//!
//! ```rust
//! use midi2::prelude::*;
//!
//! let buffer = [
//!     0x3E16_0001,
//!     0x0203_0405,
//!     0x3E26_0607,
//!     0x0809_0A0B,
//!     0x3E26_0C0D,
//!     0x0E0F_1011,
//!     0x3E26_1213,
//!     0x1415_1617,
//!     0x3E26_1819,
//!     0x1A1B_1C1D,
//!     0x3E26_1E1F,
//!     0x2021_2223,
//!     0x3E26_2425,
//!     0x2627_2829,
//!     0x3E26_2A2B,
//!     0x2C2D_2E2F,
//!     0x3E32_3031,
//!     0x0000_0000,
//! ];
//!
//! // A message can be backed with a borrowed slice of data.
//! let message: UmpMessage<&[u32]> = UmpMessage::try_from(&buffer[..]).expect("Valid data");
//!
//! // Or a vector
//! let vector_backed: UmpMessage<Vec<u32>> = message.rebuffer_into();
//!
//! // Or a fixed size array
//! let arr_backed: UmpMessage<[u32; 18]> = vector_backed
//!     .try_rebuffer_into()
//!     .expect("Buffer large enough");
//!
//! // (Or indeed a custom buffer, if you implement the right traits!)
//! ```
//! A buffer can be any data type which returns a slice of `u32` or `u8`.
//!
//! The buffer traits are already implemented for some typical standard rust types:
//! * `&[U] where U: Unit`
//! * `&mut [U] where U: Unit`
//! * `[U; SIZE] where U: Unit`
//! * `Vec<U> where U: Unit` (with the `std` feature enabled)
//!
//! The api of the message wrapper changes depending on the traits of the
//! backing buffer.
//!
//! For example `&[U]` implements [Buffer]
//! but doesn't implement [BufferMut] so messages
//! backed by a `&[U]` are imutable.
//!
//! ```compile_fail,E0277
//! use midi2::{
//!     prelude::*,
//!     channel_voice1::NoteOn,
//! };
//!
//! let mut message: NoteOn<&[u32]> = NoteOn::try_from(&[0x2D9E_753D][..]).expect("Valid data");
//!
//! // the immutable api is available
//! assert_eq!(message.note(), u7::default());
//!
//! // error[E0277]: the trait bound `&[u32]: BufferMut` is not satisfied
//! message.set_note(u7::new(0x60));
//! ```
//!
//! `[U: SIZE]` buffers implement [BufferMut], but only
//! [BufferTryResize] so any methods which require
//! a resize are aivaible only in the fallible form.
//!
//! ```rust
//! use midi2::prelude::*;
//!
//! let mut message = sysex8::Sysex8::<[u32; 64]>::new();
//! assert_eq!(message.try_set_payload(0..20), Ok(()));
//! ```
//! `Vec<U>` implements [BufferMut] and [BufferResize].
//! Messages backed with with such buffers have the most powerful api.
//!
//! ```rust
//! use midi2::prelude::*;
//!
//! let mut message = sysex8::Sysex8::<Vec<u32>>::new();
//! message.set_payload(0..20); // "cannot" fail
//! ```
//!
//! ## Implementing Custom Buffers
//!
//! Using the traits from this module it is entirely
//! possible to create a custom backing buffer.
//!
//! One potential fancy use case might be to create a non-allocating
//! resizable buffer which uses an arena allocator.

use crate::error::BufferOverflow;

/// The generic unit type contained within [Buffer] slices.
///
/// This is a sealed trait.
/// It's only implemented for [u8] and [u32].
///
/// A [Buffer] with `U = u8` is a [Bytes] buffer.
///
/// A [Buffer] with `U = u32` is a [Ump] buffer.
#[allow(private_bounds)]
pub trait Unit: Copy + UnitPrivate {
    fn zero() -> Self;
}

impl Unit for u8 {
    fn zero() -> Self {
        0x0
    }
}

impl Unit for u32 {
    fn zero() -> Self {
        0x0
    }
}

/// Generic data representation for MIDI message wrapper types.
///
/// For more info see the [buffer module docs](crate::buffer).
pub trait Buffer {
    type Unit: Unit;
    fn buffer(&self) -> &[Self::Unit];
}

/// Buffer types which own mutable data.
pub trait BufferMut: Buffer {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit];
}

/// Buffer types which are default constructible.
///
/// For more info see the [buffer module docs](crate::buffer).
// N.B. This is needed because core::default::Default
// is not implemented for arrays which are generic over size
pub trait BufferDefault {
    fn default() -> Self;
}

/// Buffer types which can support arbitrary resizing.
///
/// For more info see the [buffer module docs](crate::buffer).
pub trait BufferResize {
    fn resize(&mut self, size: usize);
}

/// Buffer types which can resize, but with a chance of failure.
///
/// Note: This trait is also implemented by buffers of a fixed size.
/// In this case `try_resize` should return Ok whenever
/// the requested size is less than or equal to the fixed
/// size of the buffer and an Err otherwise.
///
/// For more info see the [buffer module docs](crate::buffer).
pub trait BufferTryResize {
    fn try_resize(&mut self, size: usize) -> Result<(), BufferOverflow>;
}

/// Buffers with `Self::Unit = u32`.
///
/// For more info see the [buffer module docs](crate::buffer).
pub trait Ump: Buffer<Unit = u32> {}

impl<B: Buffer<Unit = u32>> Ump for B {}

/// Buffers with `Self::Unit = u8`.
///
/// For more info see the [buffer module docs](crate::buffer).
pub trait Bytes: Buffer<Unit = u8> {}

impl<B: Buffer<Unit = u8>> Bytes for B {}

impl<'a, U: Unit> Buffer for &'a [U] {
    type Unit = U;
    fn buffer(&self) -> &[Self::Unit] {
        self
    }
}

impl<'a, U: Unit> Buffer for &'a mut [U] {
    type Unit = U;
    fn buffer(&self) -> &[Self::Unit] {
        self
    }
}

impl<'a, U: Unit> BufferMut for &'a mut [U] {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        self
    }
}

impl<const SIZE: usize, U: Unit> Buffer for [U; SIZE] {
    type Unit = U;
    fn buffer(&self) -> &[Self::Unit] {
        &self[..]
    }
}

impl<const SIZE: usize, U: Unit> BufferMut for [U; SIZE] {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        &mut self[..]
    }
}

impl<const SIZE: usize, U: Unit> BufferDefault for [U; SIZE] {
    fn default() -> Self {
        [U::zero(); SIZE]
    }
}

impl<const SIZE: usize, U: Unit> BufferTryResize for [U; SIZE] {
    fn try_resize(&mut self, size: usize) -> Result<(), BufferOverflow> {
        if size > self.len() {
            Err(BufferOverflow)
        } else {
            Ok(())
        }
    }
}

#[cfg(any(feature = "std", test))]
impl<U: Unit> Buffer for std::vec::Vec<U> {
    type Unit = U;
    fn buffer(&self) -> &[Self::Unit] {
        self
    }
}

#[cfg(any(feature = "std", test))]
impl<U: Unit> BufferMut for std::vec::Vec<U> {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        self
    }
}

#[cfg(any(feature = "std", test))]
impl<U: Unit> BufferResize for std::vec::Vec<U> {
    fn resize(&mut self, size: usize) {
        self.resize(size, U::zero());
    }
}

#[cfg(any(feature = "std", test))]
impl<U: Unit> BufferDefault for std::vec::Vec<U> {
    fn default() -> Self {
        Default::default()
    }
}

pub(crate) const UNIT_ID_U8: u8 = 0;
pub(crate) const UNIT_ID_U32: u8 = 1;

pub(crate) trait UnitPrivate: Copy {
    const UNIT_ID: u8;
    fn specialise_buffer_u8(buffer: &[Self]) -> &[u8];
    fn specialise_buffer_u8_mut(buffer: &mut [Self]) -> &mut [u8];

    fn specialise_buffer_u32(buffer: &[Self]) -> &[u32];
    fn specialise_buffer_u32_mut(buffer: &mut [Self]) -> &mut [u32];
}

impl UnitPrivate for u8 {
    const UNIT_ID: u8 = 0;
    fn specialise_buffer_u8(buffer: &[Self]) -> &[u8] {
        buffer
    }
    fn specialise_buffer_u8_mut(buffer: &mut [Self]) -> &mut [u8] {
        buffer
    }
    fn specialise_buffer_u32(_: &[Self]) -> &[u32] {
        unreachable!()
    }
    fn specialise_buffer_u32_mut(_: &mut [Self]) -> &mut [u32] {
        unreachable!()
    }
}

impl UnitPrivate for u32 {
    const UNIT_ID: u8 = 1;
    fn specialise_buffer_u8(_: &[Self]) -> &[u8] {
        unreachable!()
    }
    fn specialise_buffer_u8_mut(_: &mut [Self]) -> &mut [u8] {
        unreachable!()
    }
    fn specialise_buffer_u32(buffer: &[Self]) -> &[u32] {
        buffer
    }
    fn specialise_buffer_u32_mut(buffer: &mut [Self]) -> &mut [u32] {
        buffer
    }
}

pub(crate) trait SpecialiseU32<B: Buffer> {
    fn specialise_u32(&self) -> &[u32];
    fn specialise_u32_mut(&mut self) -> &mut [u32]
    where
        B: BufferMut;
}

impl<B: Buffer> SpecialiseU32<B> for B {
    fn specialise_u32(&self) -> &[u32] {
        match B::Unit::UNIT_ID {
            UNIT_ID_U32 => <B::Unit as UnitPrivate>::specialise_buffer_u32(self.buffer()),
            _ => unreachable!(),
        }
    }
    fn specialise_u32_mut(&mut self) -> &mut [u32]
    where
        B: BufferMut,
    {
        match B::Unit::UNIT_ID {
            UNIT_ID_U32 => <B::Unit as UnitPrivate>::specialise_buffer_u32_mut(self.buffer_mut()),
            _ => unreachable!(),
        }
    }
}

pub(crate) trait SpecialiseU8<B: Buffer> {
    fn specialise_u8(&self) -> &[u8];
    fn specialise_u8_mut(&mut self) -> &mut [u8]
    where
        B: BufferMut;
}

impl<B: Buffer> SpecialiseU8<B> for B {
    fn specialise_u8(&self) -> &[u8] {
        match B::Unit::UNIT_ID {
            UNIT_ID_U8 => <B::Unit as UnitPrivate>::specialise_buffer_u8(self.buffer()),
            _ => unreachable!(),
        }
    }
    fn specialise_u8_mut(&mut self) -> &mut [u8]
    where
        B: BufferMut,
    {
        match B::Unit::UNIT_ID {
            UNIT_ID_U8 => <B::Unit as UnitPrivate>::specialise_buffer_u8_mut(self.buffer_mut()),
            _ => unreachable!(),
        }
    }
}
