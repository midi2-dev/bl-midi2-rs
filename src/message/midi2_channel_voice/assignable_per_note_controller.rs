use super::super::helpers;
use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
    util::{builder, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    index: u8,
    data: u32,
}

builder::builder!(
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    index: u8,
    data: u32
);

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b0001);
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        helpers::validate_packet(&p, Message::TYPE_CODE, Message::OP_CODE)?;
        Ok(Message {
            group: helpers::group_from_packet(&p),
            channel: helpers::channel_from_packet(&p),
            note: p.octet(2).truncate(),
            index: p.octet(3),
            data: p[1],
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
        p.set_octet(2, m.note.into());
        p.set_octet(3, m.index);
        p[1] = m.data;
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::message_traits_test;

    message_traits_test!(Message);

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x4410_2A05, 0x3691_2151])),
            Ok(Message {
                group: ux::u4::new(0x4),
                channel: ux::u4::new(0x0),
                note: ux::u7::new(0x2A),
                index: 0x05,
                data: 0x3691_2151,
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x8),
                channel: ux::u4::new(0xE),
                note: ux::u7::new(0x1A),
                index: 0xF6,
                data: 0xBE90_8008,
            }),
            Packet::from_data(&[0x481E_1AF6, 0xBE90_8008]),
        );
    }
}
