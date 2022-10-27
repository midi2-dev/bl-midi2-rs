use super::note_message;
note_message!(0b1001);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::Error,
        packet::Packet,
        util::message_traits_test,
    };
    
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
            Message::try_from(Packet::from_data(&[0x2080_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x2A90_3C58])),
            Ok(Message{
                group: ux::u4::new(0xA),
                channel: ux::u4::new(0),
                note: ux::u7::new(0x3C),
                velocity: ux::u7::new(0x58),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x3),
                channel: ux::u4::new(0xA),
                note: ux::u7::new(0x66),
                velocity: ux::u7::new(0x5A),
            }),
            Packet::from_data(&[0x239A_665A]),
        );
    }
}
