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
    position: ux::u14,
}

impl Message {
    const STATUS_CODE: u8 = 0xF2;
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        super::validate_packet(
            &p,
            Message::STATUS_CODE,
        )?;
        Ok(Message {
            group: helpers::group_from_packet(&p),
            position: helpers::concatenate(
                p.octet(2).truncate(), 
                p.octet(3).truncate(),
            )
        })
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        super::write_data_to_packet(
            &mut p, 
            m.group, 
            Message::STATUS_CODE, 
            Some(helpers::least_significant_bit(m.position).into()),
            Some(helpers::most_significant_bit(m.position).into()),
        );
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
            Message::try_from(Packet::from_data(&[0x1FF2_0000 | 0b0101_1110_0011_0001])),
            Ok(Message {
                group: ux::u4::new(0xF),
                position: ux::u14::new(0b0110001_1011110),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x1),
                position: ux::u14::new(0b0011001_0111001)
            }),
            Packet::from_data(&[0x11F2_0000 | 0b00111001_00011001]),
        );
    }
}
