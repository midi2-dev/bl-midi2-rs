use crate::{
    error::Error,
    helpers::truncate, 
    packet::Packet,
};

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    bend: ux::u14,
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message{
                group: p.nibble(1),
                channel: p.nibble(3),
                bend: (ux::u14::from(p.octet(3)) << 7) | ux::u14::from(p.octet(2)),
            }),
            Err(e) => Err(e),
        }
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error> {
    match super::validate_packet(p) {
        Ok(_) => {
            if p.nibble(2) != ux::u4::new(0b1110) {
                Err(Error::InvalidData)
            } else {
                Ok(())
            }
        },
        Err(e) => Err(e),
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        Packet::new()
            .set_nibble(0, ux::u4::new(0x2))
            .set_nibble(1, m.group)
            .set_nibble(2, ux::u4::new(0b1110))
            .set_nibble(3, m.channel)
            .set_octet(2, truncate(m.bend & ux::u14::new(0b0000000_0111111)))
            .set_octet(3, truncate(m.bend >> 7))
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_status() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x2000_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0b0010_1011_1110_0000_01101001_00110011])),
            Ok(Message {
                group: ux::u4::new(0xB),
                channel: ux::u4::new(0),
                bend: ux::u14::new(0b0110011_1101001),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x5),
                channel: ux::u4::new(0x0),
                bend: ux::u14::new(0b0011011_0111001),
            }),
            Packet::from_data(&[0b0010_0101_1110_0000_00111001_00011011]),
        );
    }
}
