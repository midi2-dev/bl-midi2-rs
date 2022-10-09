use crate::{
    error::Error,
    helpers::truncate, 
    packet::Packet,
};
use builder_derive::Builder;
use getters_derive::Getters;

#[derive(
    Clone,
    Debug, 
    PartialEq,
    Builder,
    Getters,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    pressure: ux::u7,
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message{
                group: p.nibble(1),
                channel: p.nibble(3),
                note: truncate(p.octet(2)),
                pressure: truncate(p.octet(3)),
            }),
            Err(e) => Err(e),
        }
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error> {
    match super::validate_packet(p) {
        Ok(_) => {
            if p.nibble(2) != ux::u4::new(0b1010) {
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
            .set_nibble(2, ux::u4::new(0b1010))
            .set_nibble(3, m.channel)
            .set_octet(2, m.note.into())
            .set_octet(3, m.pressure.into())
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_type() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x1000_0000])),
            Err(Error::InvalidData),
        );
    }

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
            Message::try_from(Packet::from_data(&[0x22A2_7F5D])),
            Ok(Message {
                group: ux::u4::new(0x2),
                channel: ux::u4::new(2),
                note: ux::u7::new(0x7F),
                pressure: ux::u7::new(0x5D),
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x1),
                channel: ux::u4::new(0x5),
                note: ux::u7::new(0x7F),
                pressure: ux::u7::new(0x40),
            }),
            Packet::from_data(&[0x21A5_7F40]),
        );
    }
}
