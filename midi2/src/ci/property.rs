pub trait ReadCiProperty<'a, const VERSION: u8, B: crate::buffer::Buffer>:
    crate::detail::property::Property<B>
{
    fn read(buffer: &'a B) -> Self::Type;
}

pub trait ValidateCiPropertyData<const VERSION: u8, B: crate::buffer::Buffer>:
    crate::detail::property::Property<B>
{
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData>;
}

pub trait WriteCiProperty<const VERSION: u8, B: crate::buffer::Buffer + crate::buffer::BufferMut>:
    crate::detail::property::Property<B>
{
    fn write(buffer: &mut B, v: Self::Type);
    fn default() -> Self::Type;
}

pub trait ResizeCiProperty<const VERSION: u8, B: crate::buffer::Buffer + crate::buffer::BufferMut>:
    crate::detail::property::Property<B>
{
    fn resize(buffer: &mut B, value: &Self::Type)
    where
        B: crate::buffer::BufferResize;
    fn try_resize(buffer: &mut B, value: &Self::Type) -> Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferTryResize;
}
