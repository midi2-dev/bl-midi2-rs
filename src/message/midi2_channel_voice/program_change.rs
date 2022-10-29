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
    program: ux::u7,
    bank: Option<ux::u14>,
}

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1100);
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        helpers::validate_packet(&p, Message::TYPE_CODE, Message::OP_CODE)?;
        Ok(Message {
            group: helpers::group_from_packet(&p),
            channel: helpers::channel_from_packet(&p),
            program: p.octet(4).truncate(),
            bank: match p.octet(3) & 0b0000_0001 {
                1 => Some(ux::u14::from(p.octet(6)) << 7 | ux::u14::from(p.octet(7))),
                _ => None,
            },
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
        p.set_octet(4, m.program.into());
        if let Some(v) = m.bank {
            p.set_octet(6, (v >> 7).truncate());
            p.set_octet(7, v.truncate());
        }
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
            Message::try_from(Packet::from_data(&[0x42C0_0001, 0x6600_7F7F,])),
            Ok(Message {
                group: ux::u4::new(0x2),
                channel: ux::u4::new(0x0),
                program: ux::u7::new(0x66),
                bank: Some(ux::u14::new(0b11_1111_1111_1111))
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0xD),
                program: ux::u7::new(0x7C),
                bank: None,
            }),
            Packet::from_data(&[0x40CD_0000, 0x7C00_0000,])
        )
    }
}
