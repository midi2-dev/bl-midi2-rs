use crate::{
    error::Error,
    util::Numeric, 
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
    bend: ux::u14,
}

impl Message {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(0b1110);
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        channel_voice_helpers::validate_packet(
            &p,
            Message::TYPE_CODE,
            Message::OP_CODE,
        )?;
        Ok(Message{
            group: p.nibble(1),
            channel: p.nibble(3),
            bend: (ux::u14::from(p.octet(3)) << 7) | ux::u14::from(p.octet(2)),
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
        p
            .set_octet(2, (m.bend & ux::u14::new(0b0000000_0111111)).truncate())
            .set_octet(3, (m.bend >> 7).truncate())
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
            Message::try_from(Packet::from_data(&[0b0010_1011_1110_0000_01101001_00110011])),
            Ok(Message {
                group: ux::u4::new(0xB),
                channel: ux::u4::new(0),
                bend: ux::u14::new(0b0110011_1101001),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x5),
                channel: ux::u4::new(0x0),
                bend: ux::u14::new(0b0011011_0111001),
            }),
            Packet::from_data(&[0b0010_0101_1110_0000_00111001_00011011]),
        );
    }
}
