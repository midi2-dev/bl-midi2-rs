use crate::{
    error::Error,
    helpers::truncate, 
    packet::Packet,
};
use super::attribute;
use builder_derive::Builder;

#[derive(
    Clone,
    Debug, 
    PartialEq,
    Builder,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    velocity: u16,
    attribute: Option<attribute::Attribute>,
}

impl Message {
    pub fn note(&self) -> ux::u7 { self.note }
    pub fn velocity(&self) -> u16 { self.velocity }
    pub fn attribute(&self) -> Option<attribute::Attribute> { self.attribute }
}

impl super::super::Message for Message {
    const TYPE: ux::u4 = ux::u4::new(0x4);
    fn group(&self) -> ux::u4 { self.group }
}

impl super::Message for Message {
    const OP_CODE: ux::u4 = ux::u4::new(0b1000);
    fn channel(&self) -> ux::u4 { self.channel }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        <Message as super::MessagePrivate>::validate_packet(&p)?;
        Ok(Message{
            group: p.nibble(1),
            channel: p.nibble(3),
            note: truncate(p.octet(2)),
            velocity: p.word(2),
            attribute: attribute::from_packet(&p)?,
        })
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        <Message as super::MessagePrivate>::write_data_to_packet(
            super::Data { group: m.group, channel: m.channel },
            &mut p,
        );
        p
            .set_octet(2, m.note.into())
            .set_word(2, m.velocity);
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
            Message::try_from(Packet::from_data(&[0x4040_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x408A_6301, 
                0xABCD_1234,
            ])),
            Ok(Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0xA),
                note: ux::u7::new(0x63),
                velocity: 0xABCD,
                attribute: Some(attribute::Attribute::ManufacturerSpecific(0x1234)),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x5),
                channel: ux::u4::new(0x2),
                note: ux::u7::new(0x7B),
                velocity: 0x07A0,
                attribute: None
            }),
            Packet::from_data(&[0x4582_7B00, 0x07A0_0000]),
        );
    }
}
