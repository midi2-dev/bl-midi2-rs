pub trait Property<B: crate::buffer::Buffer> {
    type Type;
}

pub trait ReadProperty<B: crate::buffer::Buffer>: Property<B> {
    fn read(buffer: &B) -> Self::Type;
    // validate that the data in the buffer represents a valid instance of the property
    fn validate(buffer: &B) -> crate::result::Result<()>;
}

pub trait WriteProperty<B: crate::buffer::Buffer + crate::buffer::BufferMut>: Property<B> {
    fn write(buffer: &mut B, v: Self::Type);
    // validate that the value represents a valid instance of the property.
    // ideally the type system should do this for us so this will
    // most often just trivially return Ok
    fn validate(v: &Self::Type) -> crate::result::Result<()>;
    fn default() -> Self::Type;
}

// properties which may require resizing the underlying buffer
// before writing the value
pub trait ResizeProperty<B: crate::buffer::Buffer + crate::buffer::BufferMut>:
    WriteProperty<B>
{
    fn resize(buffer: &mut B, size: usize)
    where
        B: crate::buffer::BufferResize;
    fn try_resize(buffer: &mut B, size: usize) -> Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferTryResize;
}
