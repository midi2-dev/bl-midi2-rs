use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
}

impl Message {
    const OP_CODE: ux::u4 = ux::u4::new(0x0);
}

impl core::convert::From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        p.set_nibble(1, m.group);
        p
    }
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        super::validate_packet(&p, Message::OP_CODE)?;
        Ok(Message { group: p.nibble(1) })
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
            Message::try_from(Packet::from_data(&[0x0700_0000])),
            Ok(Message {
                group: ux::u4::new(0x7)
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x2)
            }),
            Packet::from_data(&[0x0200_0000]),
        );
    }
}
