use super::super::helpers;
use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
    util::Truncate,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    pressure: ux::u7,
}

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1010);
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        helpers::validate_packet(&p, Message::TYPE_CODE, Message::OP_CODE)?;
        Ok(Message {
            group: p.nibble(1),
            channel: p.nibble(3),
            note: p.octet(2).truncate(),
            pressure: p.octet(3).truncate(),
        })
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        helpers::write_data_to_packet(
            Message::TYPE_CODE,
            m.group,
            Message::OP_CODE,
            m.channel,
            &mut p,
        );
        p.set_octet(2, m.note.into())
            .set_octet(3, m.pressure.into());
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::message_traits_test;

    message_traits_test!(Message);

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
