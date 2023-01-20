use crate::{
    error::Error,
    message::{helpers as message_helpers, midi2_channel_voice::helpers, Midi2Message},
    util::{builder, getter, BitOps, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    index: ux::u7,
    data: u32,
}

builder::builder!(group: ux::u4, channel: ux::u4, index: ux::u7, data: u32);

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1011);
    getter::getter!(group, ux::u4);
    getter::getter!(channel, ux::u4);
    getter::getter!(index, ux::u7);
    getter::getter!(data, u32);
    builder::builder_method!();
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: message_helpers::group_from_packet(bytes),
            channel: message_helpers::channel_from_packet(bytes),
            index: bytes[0].octet(2).truncate(),
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
        bytes[0].set_octet(2, self.index.into());
        bytes[1] = self.data;
        &bytes[..2]
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
            Message::try_from_ump(&[0x4DB7_7D00, 0x1234_5678]),
            Ok(Message {
                group: ux::u4::new(0xD),
                channel: ux::u4::new(0x7),
                index: ux::u7::new(0x7D),
                data: 0x1234_5678,
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0x9),
                index: ux::u7::new(0x30),
                data: 0x2468_1012,
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x40B9_3000, 0x2468_1012],
        );
    }
}
