use crate::{
    buffer::Ump,
    util::property::{Property, ResizeProperty},
};

pub struct TextWriteStrProperty<'a>(core::marker::PhantomData<&'a u8>);

impl<'a, B: Ump> Property<B> for TextWriteStrProperty<'a> {
    type Type = &'a str;
    fn read(_: &B) -> crate::result::Result<Self::Type> {
        // writeonly property
        unreachable!()
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        todo!()
    }
    fn default() -> Self::Type {
        ""
    }
}

impl<'a, B: Ump> ResizeProperty<B> for TextWriteStrProperty<'a> {
    fn resize(buffer: &mut B, size: usize)
    where
        B: crate::buffer::BufferResize,
    {
        todo!()
    }
    fn try_resize(buffer: &mut B, size: usize) -> Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferTryResize,
    {
        todo!()
    }
}

pub struct TextBytesIterator;

pub struct TextReadBytesProperty;

impl<B: Ump> Property<B> for TextReadBytesProperty {
    type Type = TextBytesIterator;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        todo!()
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        unreachable!()
    }
    fn default() -> Self::Type {
        unreachable!()
    }
}

#[cfg(feature = "std")]
pub struct TextReadStringProperty;

#[cfg(feature = "std")]
impl<B: Ump> Property<B> for TextReadStringProperty {
    type Type = std::string::String;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        todo!()
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        unreachable!()
    }
    fn default() -> Self::Type {
        unreachable!()
    }
}
