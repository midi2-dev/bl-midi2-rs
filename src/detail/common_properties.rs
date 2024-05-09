use crate::{
    buffer::{
        Buffer, BufferMut, SpecialiseU32, SpecialiseU8, UnitPrivate, UNIT_ID_U32, UNIT_ID_U8,
    },
    detail::{
        property::{Property, ReadProperty, WriteProperty},
        schema, BitOps,
    },
    ux::*,
};

pub struct UmpMessageTypeProperty<const TYPE: u8>;

impl<const TYPE: u8, B: Buffer> Property<B> for UmpMessageTypeProperty<TYPE> {
    type Type = ();
}

impl<'a, const TYPE: u8, B: Buffer> ReadProperty<'a, B> for UmpMessageTypeProperty<TYPE> {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            let b = buffer.buffer().specialise_u32()[0];
            if b.nibble(0) != crate::ux::u4::new(TYPE) {
                return Err(crate::error::Error::InvalidData(
                    "Incorrect ump message type",
                ));
            }
        }
        Ok(())
    }
}

impl<const TYPE: u8, B: Buffer + crate::buffer::BufferMut> WriteProperty<B>
    for UmpMessageTypeProperty<TYPE>
{
    fn write(buffer: &mut B, _v: Self::Type) {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            buffer.buffer_mut().specialise_u32_mut()[0].set_nibble(0, crate::ux::u4::new(TYPE));
        }
    }
    fn validate(_value: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

pub struct ChannelVoiceStatusProperty<const STATUS: u8>;

impl<const STATUS: u8, B: Buffer> Property<B> for ChannelVoiceStatusProperty<STATUS> {
    type Type = ();
}

impl<'a, const STATUS: u8, B: Buffer> ReadProperty<'a, B> for ChannelVoiceStatusProperty<STATUS> {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        let status = match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer().specialise_u32()[0];
                b.nibble(2)
            }
            UNIT_ID_U8 => {
                let b = buffer.buffer().specialise_u8()[0];
                b.nibble(0)
            }
            _ => unreachable!(),
        };
        if status == u4::new(STATUS) {
            Ok(())
        } else {
            Err(crate::error::Error::InvalidData("Incorrect message status"))
        }
    }
}

impl<const STATUS: u8, B: Buffer + crate::buffer::BufferMut> WriteProperty<B>
    for ChannelVoiceStatusProperty<STATUS>
{
    fn write(buffer: &mut B, _v: Self::Type) {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                buffer.buffer_mut().specialise_u32_mut()[0]
                    .set_nibble(2, crate::ux::u4::new(STATUS));
            }
            UNIT_ID_U8 => {
                buffer.buffer_mut().specialise_u8_mut()[0]
                    .set_nibble(0, crate::ux::u4::new(STATUS));
            }
            _ => unreachable!(),
        }
    }
    fn validate(_value: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

pub type ChannelProperty = HybridSchemaProperty<
    u4,
    schema::Bytes<0x0F, 0x0, 0x0>,
    schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>,
>;
pub type GroupProperty = UmpSchemaProperty<u4, schema::Ump<0x0F00_0000, 0x0, 0x0, 0x0>>;

pub struct HybridSchemaProperty<T, B: schema::BytesSchema, U: schema::UmpSchema>(
    core::marker::PhantomData<(T, B, U)>,
);

impl<
        B: Buffer,
        BytesSchema: schema::BytesSchema,
        UmpSchema: schema::UmpSchema,
        T: Default + schema::UmpSchemaRepr<UmpSchema> + schema::BytesSchemaRepr<BytesSchema>,
    > Property<B> for HybridSchemaProperty<T, BytesSchema, UmpSchema>
{
    type Type = T;
}

impl<
        'a,
        B: Buffer,
        BytesSchema: schema::BytesSchema,
        UmpSchema: schema::UmpSchema,
        T: Default + schema::UmpSchemaRepr<UmpSchema> + schema::BytesSchemaRepr<BytesSchema>,
    > ReadProperty<'a, B> for HybridSchemaProperty<T, BytesSchema, UmpSchema>
{
    fn read(buffer: &'a B) -> Self::Type {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                <T as schema::UmpSchemaRepr<UmpSchema>>::read(buffer.buffer().specialise_u32())
            }
            UNIT_ID_U8 => {
                <T as schema::BytesSchemaRepr<BytesSchema>>::read(buffer.buffer().specialise_u8())
            }
            _ => unreachable!(),
        }
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<
        B: Buffer + BufferMut,
        BytesSchema: schema::BytesSchema,
        UmpSchema: schema::UmpSchema,
        T: Default + schema::UmpSchemaRepr<UmpSchema> + schema::BytesSchemaRepr<BytesSchema>,
    > WriteProperty<B> for HybridSchemaProperty<T, BytesSchema, UmpSchema>
{
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => <T as schema::UmpSchemaRepr<UmpSchema>>::write(
                buffer.buffer_mut().specialise_u32_mut(),
                v,
            ),
            UNIT_ID_U8 => <T as schema::BytesSchemaRepr<BytesSchema>>::write(
                buffer.buffer_mut().specialise_u8_mut(),
                v,
            ),
            _ => unreachable!(),
        }
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

pub struct BytesSchemaProperty<T, B: schema::BytesSchema>(core::marker::PhantomData<(T, B)>);

impl<
        B: Buffer,
        BytesSchema: schema::BytesSchema,
        T: Default + schema::BytesSchemaRepr<BytesSchema>,
    > Property<B> for BytesSchemaProperty<T, BytesSchema>
{
    type Type = T;
}

impl<
        'a,
        B: Buffer,
        BytesSchema: schema::BytesSchema,
        T: Default + schema::BytesSchemaRepr<BytesSchema>,
    > ReadProperty<'a, B> for BytesSchemaProperty<T, BytesSchema>
{
    fn read(buffer: &'a B) -> Self::Type {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => Default::default(),
            UNIT_ID_U8 => {
                <T as schema::BytesSchemaRepr<BytesSchema>>::read(buffer.buffer().specialise_u8())
            }
            _ => unreachable!(),
        }
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<
        B: Buffer + BufferMut,
        BytesSchema: schema::BytesSchema,
        T: Default + schema::BytesSchemaRepr<BytesSchema>,
    > WriteProperty<B> for BytesSchemaProperty<T, BytesSchema>
{
    fn write(buffer: &mut B, v: Self::Type) {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U8 {
            <T as schema::BytesSchemaRepr<BytesSchema>>::write(
                buffer.buffer_mut().specialise_u8_mut(),
                v,
            )
        }
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

pub struct UmpSchemaProperty<T, B: schema::UmpSchema>(core::marker::PhantomData<(T, B)>);

impl<B: Buffer, UmpSchema: schema::UmpSchema, T: Default + schema::UmpSchemaRepr<UmpSchema>>
    Property<B> for UmpSchemaProperty<T, UmpSchema>
{
    type Type = T;
}

impl<
        'a,
        B: Buffer,
        UmpSchema: schema::UmpSchema,
        T: Default + schema::UmpSchemaRepr<UmpSchema>,
    > ReadProperty<'a, B> for UmpSchemaProperty<T, UmpSchema>
{
    fn read(buffer: &'a B) -> Self::Type {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                <T as schema::UmpSchemaRepr<UmpSchema>>::read(buffer.buffer().specialise_u32())
            }
            UNIT_ID_U8 => Default::default(),
            _ => unreachable!(),
        }
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<
        B: Buffer + BufferMut,
        UmpSchema: schema::UmpSchema,
        T: Default + schema::UmpSchemaRepr<UmpSchema>,
    > WriteProperty<B> for UmpSchemaProperty<T, UmpSchema>
{
    fn write(buffer: &mut B, v: Self::Type) {
        if <B::Unit as UnitPrivate>::UNIT_ID == UNIT_ID_U32 {
            <T as schema::UmpSchemaRepr<UmpSchema>>::write(
                buffer.buffer_mut().specialise_u32_mut(),
                v,
            )
        }
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}
