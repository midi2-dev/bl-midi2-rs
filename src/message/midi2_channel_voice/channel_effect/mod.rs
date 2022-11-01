pub mod pitch_bend;
pub mod pressure;

macro_rules! channel_effect_message {
    ($op_code:expr) => {
        use crate::message::helpers as message_helpers;
        use crate::{
            error::Error,
            message::{helpers, Midi2Message},
            util::{builder, getter},
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            data: u32,
        }

        builder::builder!(group: ux::u4, channel: ux::u4, data: u32);

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi2_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
            getter::getter!(group, ux::u4);
            getter::getter!(channel, ux::u4);
            getter::getter!(data, u32);
        }

        impl Midi2Message for Message {
            fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
                helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
            }
            fn from_ump(bytes: &[u32]) -> Self {
                Message {
                    group: message_helpers::group_from_packet(bytes),
                    channel: message_helpers::channel_from_packet(bytes),
                    data: bytes[1],
                }
            }
            fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32] {
                message_helpers::write_data(
                    Message::TYPE_CODE,
                    self.group,
                    Message::OP_CODE,
                    self.channel,
                    bytes,
                );
                bytes[1] = self.data;
                &bytes[..2]
            }
        }
    };
}

pub(crate) use channel_effect_message;

#[cfg(test)]
mod tests {
    use super::channel_effect_message;
    use crate::util::message_traits_test;

    channel_effect_message!(0b1111);

    message_traits_test!(Message);

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x6),
                channel: ux::u4::new(0x6),
                data: 0x8765_4321,
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x46F6_0000, 0x8765_4321],
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x49F9_0000, 0x5101_5202]),
            Ok(Message {
                group: ux::u4::new(0x9),
                channel: ux::u4::new(0x9),
                data: 0x5101_5202,
            }),
        );
    }

    #[test]
    fn build() {
        assert_eq!(
            Message::builder()
                .group(ux::u4::new(0xB))
                .channel(ux::u4::new(0xE))
                .data(0xBABE_BABE)
                .build(),
            Message {
                group: ux::u4::new(0xB),
                channel: ux::u4::new(0xE),
                data: 0xBABE_BABE,
            }
        )
    }

    #[test]
    fn getters() {
        let m = Message {
            group: ux::u4::new(0xB),
            channel: ux::u4::new(0xE),
            data: 0xBABE_BABE,
        };
        assert_eq!(m.group(), ux::u4::new(0xB));
        assert_eq!(m.channel(), ux::u4::new(0xE));
        assert_eq!(m.data(), 0xBABE_BABE);
    }
}
