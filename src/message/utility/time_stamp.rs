use crate::{
    error::Error,
    util::Truncate, 
    packet::{Packet, PacketMethods}, 
};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message {
    time_stamp: ux::u20,
    group: ux::u4,
}

impl Message {
    const OP_CODE: ux::u4 = ux::u4::new(0b0010);
}

impl core::convert::From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::from_data(
            &[u32::from(m.time_stamp) |  0x0020_0000],
        );
        p.set_nibble(1, m.group);
        p
    }
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        super::validate_packet(&p, Message::OP_CODE)?;
        Ok(Message {
            time_stamp: p[0].truncate(),
            group: p.nibble(1),
        })
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
            Message::try_from(Packet::from_data(&[0x0A22_ABCD])),
            Ok(Message {
                time_stamp: ux::u20::new(0x2ABCD),
                group: ux::u4::new(0xA),
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                time_stamp: ux::u20::new(0x2ABCD),
                group: ux::u4::new(0xB),
            }),
            Packet::from_data(&[0x0B22_ABCD]),
        );
    }

    #[test]
    fn time_stamp() {
        assert_eq!(
            Message { 
                time_stamp: ux::u20::new(0x314),
                group: Default::default(),
            }.time_stamp,
            ux::u20::new(0x314),
        );
    }
}
