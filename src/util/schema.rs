use crate::{
    buffer::{Bytes, Ump},
    util::converter::Converter,
};
pub struct UmpSchema<const D1: u32, const D2: u32, const D3: u32, const D4: u32>();
pub struct BytesSchema<const B1: u8, const B2: u8, const B3: u8>();

#[allow(dead_code)]
pub struct Property<T, UmpSchema, BytesSchema>(
    core::marker::PhantomData<(T, UmpSchema, BytesSchema)>,
)
where
    T: Converter<Ump, UmpSchema>,
    T: Converter<Bytes, BytesSchema>;

#[derive(Default)]
pub struct NumericalConstant<const T: u32>();
