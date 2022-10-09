use crate::{
    error::Error,
    helpers::truncate, 
    packet::Packet,
};
use super::attribute;
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
    velocity: u16,
    attribute: Option<attribute::Attribute>,
}

impl Message {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(0b1001);
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        validate_packet(&p)?;
        Ok(Message{
            group: p.nibble(1),
            channel: p.nibble(3),
            note: truncate(p.octet(2)),
            velocity: p.word(2),
            attribute: attribute::from_packet(&p)?,
        })
    }
}

fn validate_packet(p: &Packet) -> Result<(), Error> {
    super::validate_packet(p)?;
    if p.nibble(2) != Message::OP_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new()
            .set_nibble(0, Message::TYPE_CODE)
            .set_nibble(1, m.group)
            .set_nibble(2, Message::OP_CODE)
            .set_nibble(3, m.channel)
            .set_octet(2, m.note.into())
            .set_word(2, m.velocity)
            .to_owned();
        attribute::write_attribute(&mut p, m.attribute);
        p
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
            Message::try_from(Packet::from_data(&[0x4080_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4393_5000, 
                0x6666_0000,
            ])),
            Ok(Message {
                group: ux::u4::new(0x3),
                channel: ux::u4::new(0x3),
                note: ux::u7::new(0x50),
                velocity: 0x6666,
                attribute: None,
            })
        );
    }

    #[test]
    fn deserialize_attribute() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4895_6A01, 
                0xFFFF_3141,
            ])),
            Ok(Message {
                group: ux::u4::new(0x8),
                channel: ux::u4::new(0x5),
                note: ux::u7::new(0x6A),
                velocity: 0xFFFF,
                attribute: Some(attribute::Attribute::ManufacturerSpecific(0x3141)),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x6),
                channel: ux::u4::new(0x0),
                note: ux::u7::new(0x12),
                velocity: 0x00BA,
                attribute: None
            }),
            Packet::from_data(&[0x4690_1200, 0x00BA_0000]),
        );
    }

    #[test]
    fn serialize_attribute() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0x2),
                note: ux::u7::new(0x5D),
                velocity: 0x429B,
                attribute: Some(attribute::Attribute::ProfileSpecific(0x0101))
            }),
            Packet::from_data(&[0x4092_5D02, 0x429B_0101]),
        );
    }
}