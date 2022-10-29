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
    Eq,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    bend: ux::u14,
}

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1110);
}

impl core::convert::TryFrom<Packet> for Message {
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
            bend: helpers::concatenate(
                p.octet(2).truncate(), 
                p.octet(3).truncate()
            ),
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
            .set_octet(2, helpers::least_significant_bit(m.bend).into())
            .set_octet(3, helpers::most_significant_bit(m.bend).into());
        p
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
            Message::try_from(Packet::from_data(&[0b0010_1011_1110_0000_0110_1001_0011_0011])),
            Ok(Message {
                group: ux::u4::new(0xB),
                channel: ux::u4::new(0),
                bend: ux::u14::new(0b01_1001_1110_1001),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x5),
                channel: ux::u4::new(0x0),
                bend: ux::u14::new(0b00_1101_1011_1001),
            }),
            Packet::from_data(&[0b0010_0101_1110_0000_0011_1001_0001_1011]),
        );
    }
}
