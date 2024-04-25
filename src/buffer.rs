pub(crate) const UNIT_ID_U8: u8 = 0;
pub(crate) const UNIT_ID_U32: u8 = 1;

pub(crate) trait UnitPrivate: Copy {
    const UNIT_ID: u8;

    fn specialise_u32(&self) -> &u32;
    fn specialise_u32_mut(&mut self) -> &mut u32;

    fn specialise_u8(&self) -> &u8;
    fn specialise_u8_mut(&mut self) -> &mut u8;
}

impl UnitPrivate for u8 {
    const UNIT_ID: u8 = 0;

    fn specialise_u32(&self) -> &u32 {
        unreachable!()
    }
    fn specialise_u32_mut(&mut self) -> &mut u32 {
        unreachable!()
    }

    fn specialise_u8(&self) -> &u8 {
        self
    }
    fn specialise_u8_mut(&mut self) -> &mut u8 {
        self
    }
}

impl UnitPrivate for u32 {
    const UNIT_ID: u8 = 1;

    fn specialise_u32(&self) -> &u32 {
        self
    }
    fn specialise_u32_mut(&mut self) -> &mut u32 {
        self
    }

    fn specialise_u8(&self) -> &u8 {
        unreachable!()
    }
    fn specialise_u8_mut(&mut self) -> &mut u8 {
        unreachable!()
    }
}

#[allow(private_bounds)]
pub trait Unit: UnitPrivate {
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

pub trait BufferResizable {
    fn resize(&mut self, size: usize);
}

pub trait BufferFixedSize {}

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

impl<const SIZE: usize, U: Unit> BufferFixedSize for [U; SIZE] {}

impl<U: Unit> Buffer for std::vec::Vec<U> {
    type Unit = U;
    fn buffer(&self) -> &[Self::Unit] {
        self
    }
}

impl<U: Unit> BufferMut for std::vec::Vec<U> {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        self
    }
}

impl<U: Unit> BufferResizable for std::vec::Vec<U> {
    fn resize(&mut self, size: usize) {
        self.resize(size, U::zero());
    }
}

impl<U: Unit> BufferDefault for std::vec::Vec<U> {
    fn default() -> Self {
        Default::default()
    }
}
