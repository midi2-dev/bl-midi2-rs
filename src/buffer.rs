const UNIT_ID_U8: u8 = 0;
const UNIT_ID_U32: u8 = 1;

trait UnitPrivate {
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
pub trait Unit: Clone + UnitPrivate {}
impl Unit for u8 {}
impl Unit for u32 {}

pub trait Buffer {
    type Unit: Unit;
    fn buffer(&self) -> &[Self::Unit];
}

pub trait BufferMut: Buffer {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit];
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

impl Buffer for [u32; 1] {
    type Unit = u32;
    fn buffer(&self) -> &[Self::Unit] {
        &self[..]
    }
}

impl BufferMut for [u32; 1] {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        &mut self[..]
    }
}

impl Buffer for [u32; 2] {
    type Unit = u32;
    fn buffer(&self) -> &[Self::Unit] {
        &self[..]
    }
}

impl BufferMut for [u32; 2] {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        &mut self[..]
    }
}

impl Buffer for [u32; 3] {
    type Unit = u32;
    fn buffer(&self) -> &[Self::Unit] {
        &self[..]
    }
}

impl BufferMut for [u32; 3] {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        &mut self[..]
    }
}

impl Buffer for [u32; 4] {
    type Unit = u32;
    fn buffer(&self) -> &[Self::Unit] {
        &self[..]
    }
}

impl BufferMut for [u32; 4] {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        &mut self[..]
    }
}

impl Buffer for [u8; 2] {
    type Unit = u8;
    fn buffer(&self) -> &[Self::Unit] {
        &self[..]
    }
}

impl BufferMut for [u8; 2] {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        &mut self[..]
    }
}

impl Buffer for [u8; 3] {
    type Unit = u8;
    fn buffer(&self) -> &[Self::Unit] {
        &self[..]
    }
}

impl BufferMut for [u8; 3] {
    fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
        &mut self[..]
    }
}

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
