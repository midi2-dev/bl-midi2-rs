use crate::{
    error::Error,
    message::{helpers as message_helpers, midi2_channel_voice::helpers, Midi2Message},
    util::{builder, getter, BitOps, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    index: u8,
    data: u32,
}

builder::builder!(
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    index: u8,
    data: u32
);

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b0001);
    getter::getter!(group, ux::u4);
    getter::getter!(channel, ux::u4);
    getter::getter!(note, ux::u7);
    getter::getter!(index, u8);
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
            note: bytes[0].octet(2).truncate(),
            index: bytes[0].octet(3),
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
        bytes[0].set_octet(2, self.note.into());
        bytes[0].set_octet(3, self.index);
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
            Message::try_from_ump(&[0x4410_2A05, 0x3691_2151]),
            Ok(Message {
                group: ux::u4::new(0x4),
                channel: ux::u4::new(0x0),
                note: ux::u7::new(0x2A),
                index: 0x05,
                data: 0x3691_2151,
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x8),
                channel: ux::u4::new(0xE),
                note: ux::u7::new(0x1A),
                index: 0xF6,
                data: 0xBE90_8008,
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x481E_1AF6, 0xBE90_8008],
        );
    }
}
