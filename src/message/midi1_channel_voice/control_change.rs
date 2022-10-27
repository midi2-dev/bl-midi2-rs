use crate::{
    error::Error,
    util::Truncate, 
    packet::{Packet, PacketMethods},
};
use super::super::helpers;

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    controller: ux::u7,
    value: ux::u7,
}

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1011);
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        helpers::validate_packet(
            &p,
            Message::TYPE_CODE,
            Message::OP_CODE,
        )?;
        Ok(Message{
            group: p.nibble(1),
            channel: p.nibble(3),
            controller: p.octet(2).truncate(),
            value: p.octet(3).truncate(),
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
        p
            .set_octet(2, m.controller.into())
            .set_octet(3, m.value.into())
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::message_traits_test;
    
    message_traits_test!(Message);

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
            Message::try_from(Packet::from_data(&[0x21BF_010A])),
            Ok(Message {
                group: ux::u4::new(0x1),
                channel: ux::u4::new(15),
                controller: ux::u7::new(1),
                value: ux::u7::new(10),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x9),
                channel: ux::u4::new(0x0),
                controller: ux::u7::new(0x30),
                value: ux::u7::new(0x32),
            }),
            Packet::from_data(&[0x29B0_3032]),
        );
    }
}
