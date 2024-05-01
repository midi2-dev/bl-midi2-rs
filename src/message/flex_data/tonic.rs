use crate::{
    error::Error,
    numeric_types::*,
    result::Result,
    util::{schema, BitOps},
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

// impl Property<Tonic, UmpSchema<0x0, 0x0F00_0000, 0x0, 0x0>, ()> for Ump {
//     fn get(data: &[<Ump as Buffer>::Data]) -> Tonic {
//         Tonic::from_nibble(data[1].nibble(1)).unwrap()
//     }
//     fn write(data: &mut [<Ump as Buffer>::Data], v: Tonic) {
//     }
//     fn validate(data: &[<Self as Buffer>::Data]) -> Result<()> {
//         Tonic::from_nibble(data[1].nibble(1))?;
//         Ok(())
//     }
// }

impl schema::UmpSchemaRepr<schema::Ump<0x0, 0x0F00_0000, 0x0, 0x0>> for Tonic {
    fn write(buffer: &mut [u32], value: Self) -> Result<()> {
        buffer[1].set_nibble(1, value.into_nibble());
        Ok(())
    }
    fn read(buffer: &[u32]) -> Result<Self> {
        Ok(Tonic::from_nibble(buffer[1].nibble(1))?)
    }
}

impl schema::UmpSchemaRepr<schema::Ump<0x0, 0x0, 0x0, 0x0F00_0000>> for Tonic {
    fn write(buffer: &mut [u32], value: Self) -> Result<()> {
        buffer[3].set_nibble(1, value.into_nibble());
        Ok(())
    }
    fn read(buffer: &[u32]) -> Result<Self> {
        Ok(Tonic::from_nibble(buffer[3].nibble(1))?)
    }
}
impl core::default::Default for Tonic {
    /// Default value is [Tonic::C]
    fn default() -> Self {
        Tonic::C
    }
}

impl Tonic {
    fn from_nibble(nibble: u4) -> Result<Self> {
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
            _ => Err(Error::InvalidData("Couldn't interpret Tonic field")),
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
