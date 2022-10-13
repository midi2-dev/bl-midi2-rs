use crate::{
    packet::Packet,
    error::Error,
    util::truncate,
};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
)]
#[non_exhaustive]
pub enum Attribute {
    ManufacturerSpecific(u16),
    ProfileSpecific(u16),
    Pitch7_9 { note: ux::u7, pitch_up: ux::u9 },
}

pub fn from_packet(p: &Packet) -> Result<Option<Attribute>, Error> {
    match p.octet(3) {
        0x0 => Ok(None),
        0x1 => Ok(Some(Attribute::ManufacturerSpecific(p.word(3)))),
        0x2 => Ok(Some(Attribute::ProfileSpecific(p.word(3)))),
        0x3 => Ok(Some(Attribute::Pitch7_9 {
            note: truncate(p.word(3) >> 9),
            pitch_up: truncate(p.word(3)),
        })),
        _ => Err(Error::InvalidData),
    }
}

pub fn write_attribute(p: &mut Packet, attr: Option<Attribute>) {
    match attr {
        None => {
            p.set_octet(3, 0x0);
        },
        Some(a) => match a {
            Attribute::ManufacturerSpecific(d) => {
                p.set_octet(3, 0x1);
                p.set_word(3, d);
            },
            Attribute::ProfileSpecific(d) => {
                p.set_octet(3, 0x2);
                p.set_word(3, d);
            },
            Attribute::Pitch7_9{ note, pitch_up } => {
                let d = (u16::from(note) << 9) | u16::from(pitch_up);
                p.set_octet(3, 0x3);
                p.set_word(3, d);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_packet_invalid() {
        assert_eq!(
            from_packet(&Packet::from_data(&[0x0000_0004])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn from_packet_none() {
        assert_eq!(
            from_packet(&Default::default()),
            Ok(None),
        );
    }
    
    #[test]
    fn from_packet_generic() {
        assert_eq!(
            from_packet(&Packet::from_data(&[0x0000_0001, 0x0000_1234])),
            Ok(Some(Attribute::ManufacturerSpecific(0x1234))),
        );
    }

    #[test]
    fn from_packet_pitch7_9() {
        assert_eq!(
            from_packet(&Packet::from_data(&[0x0000_0003, 0b0000_0000_0000_0000_0011_0011_0011_0011])),
            Ok(Some(Attribute::Pitch7_9{ note: ux::u7::new(0b0011001), pitch_up: ux::u9::new(0b100110011) })),
        );
    }
    
    #[test]
    fn write_attribute_none() {
        let mut p: Packet = Default::default();
        write_attribute(&mut p, None);
        assert_eq!(p, Default::default());
    }

    #[test]
    fn write_attribute_generic() {
        let mut p: Packet = Default::default();
        write_attribute(&mut p, Some(Attribute::ProfileSpecific(0x0666)));
        assert_eq!(p, Packet::from_data(&[0x0000_0002, 0x0000_0666]));
    }

    #[test]
    fn write_attribute_pitch7_9() {
        let mut p: Packet = Default::default();
        let attribute = Attribute::Pitch7_9{
            note: ux::u7::new(0b1011100), 
            pitch_up: ux::u9::new(0b100010111)
        };
        write_attribute(&mut p, Some(attribute));
        assert_eq!(p, Packet::from_data(&[0x0000_0003, 0b0000_0000_0000_0000_1011_1001_0001_0111]));
    }
}