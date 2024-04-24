pub struct UmpMessageTypeProperty<const TYPE: u8>;

impl<const TYPE: u8, B: crate::buffer::Ump> crate::util::property::Property<B>
    for UmpMessageTypeProperty<TYPE>
{
    type Type = u8;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        todo!()
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        todo!()
    }
    fn default() -> Self::Type {
        0x0
    }
}

pub struct UtilityStatusProperty<const TYPE: u8>;

impl<const TYPE: u8, B: crate::buffer::Ump> crate::util::property::Property<B>
    for UtilityStatusProperty<TYPE>
{
    type Type = u8;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        todo!()
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        todo!()
    }
    fn default() -> Self::Type {
        0x0
    }
}
