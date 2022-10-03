use crate::{
    error::Error,
    helpers::mask, 
    packet::Packet,
};
use builder_derive::Builder;
use getters_derive::Getters;

#[derive(
    Clone,
    Debug, 
    Default, 
    PartialEq,
    Builder,
    Getters,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    velocity: ux::u7,
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        match validate_packet(&p) {
            Ok(_) => Ok(Message{
                group: p.nibble(1),
                channel: p.nibble(3),
                note: mask(p.octet(2)),
                velocity: mask(p.octet(3)),
            }),
            Err(e) => Err(e),
        }
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error> {
    match super::validate_packet(p) {
        Ok(_) => {
            if p.nibble(2) != ux::u4::new(0b1000) {
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
            .set_nibble(2, ux::u4::new(0x8))
            .set_nibble(3, m.channel)
            .set_octet(2, m.note.into())
            .set_octet(3, m.velocity.into())
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
            Message::try_from(Packet::from_data(&[0x2040_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x2A80_3C58])),
            Ok(Message{
                group: ux::u4::new(0xA),
                channel: ux::u4::new(0),
                note: ux::u7::new(0x3C),
                velocity: ux::u7::new(0x58),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x3),
                channel: ux::u4::new(0xA),
                note: ux::u7::new(0x66),
                velocity: ux::u7::new(0x5A),
            }),
            Packet::from_data(&[0x238A_665A]),
        );
    }
}
