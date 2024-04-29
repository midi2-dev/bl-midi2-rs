use crate::error::BufferOverflow;

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

pub trait Buffer {
    type Unit: Unit;
    fn buffer(&self) -> &[Self::Unit];
}

pub trait BufferMut: Buffer {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit];
}

// N.B. This is needed because core::default::Default
// is not implemented for arrays which are generic over size
pub trait BufferDefault {
    fn default() -> Self;
}

pub trait BufferResize {
    fn resize(&mut self, size: usize);
}

/// This trait can be implemented by buffers with
/// fallible memory allocation.
///
/// It can also be implemented by buffers of a fixed size.
/// In this case `try_resize` should return Ok whenever
/// the requested size is less than or equal to the fixed
/// size of the buffer and an Err otherwise.
pub trait BufferTryResize {
    fn try_resize(&mut self, size: usize) -> Result<(), BufferOverflow>;
}

pub trait Ump: Buffer<Unit = u32> {}

impl<B: Buffer<Unit = u32>> Ump for B {}

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
