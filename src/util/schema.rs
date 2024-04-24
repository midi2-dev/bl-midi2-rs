pub struct UmpSchema<const D1: u32, const D2: u32, const D3: u32, const D4: u32>();
pub struct BytesSchema<const B1: u8, const B2: u8, const B3: u8>();

pub trait Schema {}
impl<const B1: u8, const B2: u8, const B3: u8> Schema for BytesSchema<B1, B2, B3> {}
impl<const D1: u32, const D2: u32, const D3: u32, const D4: u32> Schema
    for UmpSchema<D1, D2, D3, D4>
{
}
