pub mod assignable;
pub mod registered;
pub mod relative_assignable;
pub mod relative_registered;

macro_rules! controller_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            message::{helpers as message_helpers, midi2_channel_voice::helpers, Midi2Message},
            util::{builder, getter, BitOps, Truncate},
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            bank: ux::u7,
            index: ux::u7,
            data: u32,
        }

        builder::builder!(
            group: ux::u4,
            channel: ux::u4,
            bank: ux::u7,
            index: ux::u7,
            data: u32
        );

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi2_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
            getter::getter!(group, ux::u4);
            getter::getter!(channel, ux::u4);
            getter::getter!(bank, ux::u7);
            getter::getter!(index, ux::u7);
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
                    bank: bytes[0].octet(2).truncate(),
                    index: bytes[0].octet(3).truncate(),
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
                bytes[0]
                    .set_octet(2, self.bank.into())
                    .set_octet(3, self.index.into());
                bytes[1] = self.data;
                &bytes[..2]
            }
        }
    };
}

pub(crate) use controller_message;

#[cfg(test)]
mod tests {
    use super::controller_message;
    use crate::util::message_traits_test;

    controller_message!(0b1111);

    message_traits_test!(Message);

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x3),
                channel: ux::u4::new(0xC),
                bank: ux::u7::new(0x51),
                index: ux::u7::new(0x3F),
                data: 0xF00F_F00F,
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x43FC_513F, 0xF00F_F00F],
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x4FFB_1011, 0xBABE_BABE]),
            Ok(Message {
                group: ux::u4::new(0xF),
                channel: ux::u4::new(0xB),
                bank: ux::u7::new(0x10),
                index: ux::u7::new(0x11),
                data: 0xBABE_BABE,
            })
        );
    }

    #[test]
    fn build() {
        assert_eq!(
            Message::builder()
                .group(ux::u4::new(0x2))
                .channel(ux::u4::new(0x1))
                .bank(ux::u7::new(0x39))
                .index(ux::u7::new(0x42))
                .data(0x1234_5678)
                .build(),
            Message {
                group: ux::u4::new(0x2),
                channel: ux::u4::new(0x1),
                bank: ux::u7::new(0x39),
                index: ux::u7::new(0x42),
                data: 0x1234_5678,
            }
        )
    }

    #[test]
    fn getters() {
        let m = Message {
            group: ux::u4::new(0x2),
            channel: ux::u4::new(0x1),
            bank: ux::u7::new(0x39),
            index: ux::u7::new(0x42),
            data: 0x1234_5678,
        };
        assert_eq!(m.group(), ux::u4::new(0x2));
        assert_eq!(m.channel(), ux::u4::new(0x1));
        assert_eq!(m.bank(), ux::u7::new(0x39));
        assert_eq!(m.index(), ux::u7::new(0x42));
        assert_eq!(m.data(), 0x1234_5678);
    }
}
