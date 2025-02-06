use crate::{
    detail::{schema, BitOps},
    ux::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tonic {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    NonStandard,
}

pub struct TonicProperty<S: schema::UmpSchema>(S);

impl<S: schema::UmpSchema, B: crate::buffer::Ump> crate::detail::property::Property<B>
    for TonicProperty<S>
{
    type Type = Tonic;
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B>
    for TonicProperty<schema::Ump<0x0, 0x0F00_0000, 0x0, 0x0>>
{
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        Tonic::from_nibble(buffer.buffer()[1].nibble(1))?;
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        Tonic::from_nibble(buffer.buffer()[1].nibble(1)).unwrap()
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for TonicProperty<schema::Ump<0x0, 0x0F00_0000, 0x0, 0x0>>
{
    fn validate(_: &Tonic) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[1].set_nibble(1, v.into_nibble());
    }
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B>
    for TonicProperty<schema::Ump<0x0, 0x0, 0x0, 0x0F00_0000>>
{
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        Tonic::from_nibble(buffer.buffer()[3].nibble(1))?;
        Ok(())
    }
    fn read(buffer: &'a B) -> Self::Type {
        Tonic::from_nibble(buffer.buffer()[3].nibble(1)).unwrap()
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for TonicProperty<schema::Ump<0x0, 0x0, 0x0, 0x0F00_0000>>
{
    fn validate(_: &Tonic) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        buffer.buffer_mut()[3].set_nibble(1, v.into_nibble());
    }
}

impl core::default::Default for Tonic {
    /// Default value is [Tonic::C]
    fn default() -> Self {
        Tonic::C
    }
}

impl Tonic {
    fn from_nibble(nibble: u4) -> Result<Self, crate::error::InvalidData> {
        use Tonic::*;
        match u8::from(nibble) {
            0x0 => Ok(NonStandard),
            0x1 => Ok(A),
            0x2 => Ok(B),
            0x3 => Ok(C),
            0x4 => Ok(D),
            0x5 => Ok(E),
            0x6 => Ok(F),
            0x7 => Ok(G),
            _ => Err(crate::error::InvalidData("Couldn't interpret Tonic field")),
        }
    }

    fn into_nibble(self) -> u4 {
        use Tonic::*;
        u4::new(match self {
            A => 0x1,
            B => 0x2,
            C => 0x3,
            D => 0x4,
            E => 0x5,
            F => 0x6,
            G => 0x7,
            NonStandard => 0x0,
        })
    }
}
