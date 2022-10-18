pub type Message = super::Message<0b1010>;

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
    fn wrong_status() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x4090_0000])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[
                0x4CA5_3A00, 
                0xABCD_EF01,
            ])),
            Ok(Message {
                group: ux::u4::new(0xC),
                channel: ux::u4::new(0x5),
                note: ux::u7::new(0x3A),
                data: 0xABCD_EF01,
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0xF),
                channel: ux::u4::new(0x2),
                note: ux::u7::new(0x38),
                data: 0x2468_1012,
            }),
            Packet::from_data(&[
                0x4FA2_3800,
                0x2468_1012,
            ])
        )
    }
}