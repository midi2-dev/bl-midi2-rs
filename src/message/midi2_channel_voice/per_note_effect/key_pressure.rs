use super::per_note_effect_message;
per_note_effect_message!(0b1010);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::Error, util::message_traits_test};

    message_traits_test!(Message);

    #[test]
    fn wrong_status() {
        assert_eq!(
            Message::try_from_ump(&[0x4090_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x4CA5_3A00, 0xABCD_EF01,]),
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
            Message {
                group: ux::u4::new(0xF),
                channel: ux::u4::new(0x2),
                note: ux::u7::new(0x38),
                data: 0x2468_1012,
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x4FA2_3800, 0x2468_1012,],
        )
    }
}
