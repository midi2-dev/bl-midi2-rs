pub trait Property<B: crate::buffer::Buffer> {
    type Read;
    type Write;
    fn read(buffer: &B) -> crate::result::Result<Self::Read>;
    fn write(buffer: &mut B, v: Self::Write) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut;
    fn default() -> Self::Write;
}

pub trait ResizeProperty<B: crate::buffer::Buffer>: Property<B> {
    fn resize(buffer: &mut B, size: usize)
    where
        B: crate::buffer::BufferResize;
    fn try_resize(buffer: &mut B, size: usize) -> Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferTryResize;
}
