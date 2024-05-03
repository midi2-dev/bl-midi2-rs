pub trait Property<B: crate::buffer::Buffer> {
    type Type;
    fn read(buffer: &B) -> crate::result::Result<Self::Type>;
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut;
    fn default() -> Self::Type;
}
