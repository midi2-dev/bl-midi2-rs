pub type Message = super::Message<0b1001>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::Error,
        packet::{Packet, PacketMethods},
        util::message_traits_test,
    };
    use super::super::attribute;

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
            Message::try_from(Packet::from_data(&[0x4080_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4393_5000, 
                0x6666_0000,
            ])),
            Ok(Message {
                group: ux::u4::new(0x3),
                channel: ux::u4::new(0x3),
                note: ux::u7::new(0x50),
                velocity: 0x6666,
                attribute: None,
            })
        );
    }

    #[test]
    fn deserialize_attribute() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4895_6A01, 
                0xFFFF_3141,
            ])),
            Ok(Message {
                group: ux::u4::new(0x8),
                channel: ux::u4::new(0x5),
                note: ux::u7::new(0x6A),
                velocity: 0xFFFF,
                attribute: Some(attribute::Attribute::ManufacturerSpecific(0x3141)),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x6),
                channel: ux::u4::new(0x0),
                note: ux::u7::new(0x12),
                velocity: 0x00BA,
                attribute: None
            }),
            Packet::from_data(&[0x4690_1200, 0x00BA_0000]),
        );
    }

    #[test]
    fn serialize_attribute() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0x2),
                note: ux::u7::new(0x5D),
                velocity: 0x429B,
                attribute: Some(attribute::Attribute::ProfileSpecific(0x0101))
            }),
            Packet::from_data(&[0x4092_5D02, 0x429B_0101]),
        );
    }
}