use crate::{
    buffer::{UnitPrivate, UNIT_ID_U32, UNIT_ID_U8},
    numeric_types::*,
    util::BitOps,
};

pub struct UmpMessageTypeProperty<const TYPE: u8>;

impl<const TYPE: u8, B: crate::buffer::Buffer> crate::util::property::Property<B>
    for UmpMessageTypeProperty<TYPE>
{
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            let b = buffer.buffer()[0].specialise_u32();
            if b.nibble(0) != crate::u4::new(TYPE) {
                return Err(crate::Error::InvalidData("Incorrect ump message type"));
            }
        }
        Ok(())
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            let b = buffer.buffer_mut()[0].specialise_u32_mut();
            b.set_nibble(0, crate::u4::new(TYPE));
        }
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

pub struct ChannelVoiceStatusProperty<const STATUS: u8>;

impl<const STATUS: u8, B: crate::buffer::Buffer> crate::util::property::Property<B>
    for ChannelVoiceStatusProperty<STATUS>
{
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        let status = match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer()[0].specialise_u32();
                b.nibble(2)
            }
            UNIT_ID_U8 => {
                let b = buffer.buffer()[0].specialise_u8();
                b.nibble(0)
            }
            _ => unreachable!(),
        };
        if status == u4::new(STATUS) {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message status"))
        }
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer_mut()[0].specialise_u32_mut();
                b.set_nibble(2, crate::u4::new(STATUS));
            }
            UNIT_ID_U8 => {
                let b = buffer.buffer_mut()[0].specialise_u8_mut();
                b.set_nibble(0, crate::u4::new(STATUS));
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

pub struct ChannelProperty;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for ChannelProperty {
    type Type = u4;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer()[0].specialise_u32();
                Ok(b.nibble(3))
            }
            UNIT_ID_U8 => {
                let b = buffer.buffer()[0].specialise_u8();
                Ok(b.nibble(1))
            }
            _ => unreachable!(),
        }
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer_mut()[0].specialise_u32_mut();
                b.set_nibble(3, v);
            }
            UNIT_ID_U8 => {
                let b = buffer.buffer_mut()[0].specialise_u8_mut();
                b.set_nibble(1, v);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn default() -> Self::Type {
        u4::new(0x0)
    }
}

pub struct GroupProperty;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for GroupProperty {
    type Type = u4;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer()[0].specialise_u32();
                Ok(b.nibble(1))
            }
            UNIT_ID_U8 => Ok(<Self as crate::util::property::Property<B>>::default()),
            _ => unreachable!(),
        }
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            let b = buffer.buffer_mut()[0].specialise_u32_mut();
            b.set_nibble(1, v);
        }
        Ok(())
    }
    fn default() -> Self::Type {
        u4::new(0x0)
    }
}
