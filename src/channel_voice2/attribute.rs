use crate::detail::{property, BitOps};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Attribute {
    ManufacturerSpecific(u16),
    ProfileSpecific(u16),
    Pitch7_9(crate::num::Fixed7_9),
    // Pitch7_25 is also defined
    // Pitch7_14??
}

const ERR_INVALID_NOTE_ATTRIBUTE: &str = "Couldn't interpret note attribute";

pub fn validate_ump(bytes: &[u32]) -> Result<(), crate::error::InvalidData> {
    match bytes[0].octet(3) {
        0x0 => Ok(()),
        0x1 => Ok(()),
        0x2 => Ok(()),
        0x3 => Ok(()),
        _ => Err(crate::error::InvalidData(ERR_INVALID_NOTE_ATTRIBUTE)),
    }
}

pub fn from_ump(buffer: &[u32]) -> Option<Attribute> {
    match buffer[0].octet(3) {
        0x0 => None,
        0x1 => Some(Attribute::ManufacturerSpecific(buffer[1].word(1))),
        0x2 => Some(Attribute::ProfileSpecific(buffer[1].word(1))),
        0x3 => Some(Attribute::Pitch7_9(crate::num::Fixed7_9::from_bits(
            buffer[1].word(1),
        ))),
        _ => panic!("Invalid status"),
    }
}

pub fn write_attribute(bytes: &mut [u32], attr: Option<Attribute>) -> &mut [u32] {
    match attr {
        None => {
            bytes[0].set_octet(3, 0x0);
        }
        Some(a) => match a {
            Attribute::ManufacturerSpecific(d) => {
                bytes[0].set_octet(3, 0x1);
                bytes[1].set_word(1, d);
            }
            Attribute::ProfileSpecific(d) => {
                bytes[0].set_octet(3, 0x2);
                bytes[1].set_word(1, d);
            }
            Attribute::Pitch7_9(fixed) => {
                bytes[0].set_octet(3, 0x3);
                bytes[1].set_word(1, fixed.to_bits());
            }
        },
    }
    &mut bytes[..2]
}

pub struct AttributeProperty;

impl<B: crate::buffer::Ump> property::Property<B> for AttributeProperty {
    type Type = Option<Attribute>;
}

impl<'a, B: crate::buffer::Ump> property::ReadProperty<'a, B> for AttributeProperty {
    fn read(buffer: &'a B) -> Self::Type {
        from_ump(buffer.buffer())
    }
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        validate_ump(buffer.buffer())
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> property::WriteProperty<B>
    for AttributeProperty
{
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn write(buffer: &mut B, v: Self::Type) {
        write_attribute(buffer.buffer_mut(), v);
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

// impl Property<Option<Attribute>, UmpSchema<0x0000_00FF, 0x0000_FFFF, 0x0, 0x0>, ()> for Ump {
//     fn get(data: &[<Ump as Buffer>::Data]) -> Option<Attribute> {
//     }
//     fn write(data: &mut [<Ump as Buffer>::Data], v: Option<Attribute>) {
//     }
//     fn validate(data: &[<Ump as Buffer>::Data]) -> Result<()> {
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn try_from_ump(bytes: &[u32]) -> Result<Option<Attribute>, crate::error::InvalidData> {
        validate_ump(bytes)?;
        Ok(from_ump(bytes))
    }

    #[test]
    fn from_packet_invalid() {
        assert_eq!(
            try_from_ump(&[0x0000_0004]),
            Err(crate::error::InvalidData(ERR_INVALID_NOTE_ATTRIBUTE)),
        );
    }

    #[test]
    fn from_packet_none() {
        assert_eq!(try_from_ump(&[0x0, 0x0]), Ok(None),);
    }

    #[test]
    fn from_packet_generic() {
        assert_eq!(
            try_from_ump(&[0x0000_0001, 0x0000_1234]),
            Ok(Some(Attribute::ManufacturerSpecific(0x1234))),
        );
    }

    #[test]
    fn from_packet_pitch7_9() {
        assert_eq!(
            try_from_ump(&[0x0000_0003, 0b0000_0000_0000_0000_0011_0011_0011_0011]),
            Ok(Some(Attribute::Pitch7_9(crate::num::Fixed7_9::from_bits(
                0b0011_0011_0011_0011
            )))),
        );
    }

    #[test]
    fn write_attribute_none() {
        assert_eq!(write_attribute(&mut [0x0, 0x0], None), &[0x0, 0x0],);
    }

    #[test]
    fn write_attribute_generic() {
        assert_eq!(
            write_attribute(&mut [0x0, 0x0], Some(Attribute::ProfileSpecific(0x0666))),
            &[0x0000_0002, 0x0000_0666],
        );
    }

    #[test]
    fn write_attribute_pitch7_9() {
        let attribute = Attribute::Pitch7_9(crate::num::Fixed7_9::from_bits(0b1011_1001_0001_0111));
        assert_eq!(
            write_attribute(&mut [0x0, 0x0], Some(attribute)),
            &[0x0000_0003, 0b0000_0000_0000_0000_1011_1001_0001_0111]
        );
    }
}
