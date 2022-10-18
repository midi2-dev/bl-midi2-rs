use crate::{
    error::Error,
    util::Truncate, 
    packet::{Packet, PacketMethods},
};
use super::super::channel_voice_helpers;

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    index: ux::u7,
    data: u32,
}

impl Message {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(0b1011);
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        channel_voice_helpers::validate_packet(
            &p,
            Message::TYPE_CODE,
            Message::OP_CODE,
        )?;
        Ok(Message {
            group: channel_voice_helpers::group_from_packet(&p),
            channel: channel_voice_helpers::channel_from_packet(&p),
            index: p.octet(2).truncate(),
            data: p[1],
        })
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        channel_voice_helpers::write_data_to_packet(
            Message::TYPE_CODE,
            m.group,
            Message::OP_CODE,
            m.channel,
            &mut p,
        );
        p.set_octet(2, m.index.into());
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
            Message::try_from(Packet::from_data(&[0x4DB7_7D00, 0x1234_5678])),
            Ok(Message {
                group: ux::u4::new(0xD),
                channel: ux::u4::new(0x7),
                index: ux::u7::new(0x7D),
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0x9),
                index: ux::u7::new(0x30),
                data: 0x2468_1012,
            }),
            Packet::from_data(&[0x40B9_3000, 0x2468_1012]),
        );
    }
}