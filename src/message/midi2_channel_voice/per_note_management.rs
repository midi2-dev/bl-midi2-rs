use crate::{
    error::Error,
    message::{helpers as message_helpers, Midi2Message},
    util::{builder, getter, BitOps, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    detach: bool,
    reset: bool,
}

builder::builder!(
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    detach: bool,
    reset: bool
);

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1111);
    getter::getter!(group, ux::u4);
    getter::getter!(channel, ux::u4);
    getter::getter!(note, ux::u7);
    getter::getter!(detach, bool);
    getter::getter!(reset, bool);
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        message_helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: message_helpers::group_from_packet(bytes),
            channel: message_helpers::channel_from_packet(bytes),
            note: bytes[0].octet(2).truncate(),
            detach: 0b0000_0010 & bytes[0].octet(3) == 0b0000_0010,
            reset: 0b0000_0001 & bytes[0].octet(3) == 0b0000_0001,
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
        let mut flags = 0x0_u8;
        if self.detach {
            flags |= 0b0000_0010;
        }
        if self.reset {
            flags |= 0b0000_0001;
        }
        bytes[0].set_octet(3, flags);
        &bytes[..1]
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
            Message::try_from_ump(&[0x41FF_4003]),
            Ok(Message {
                group: ux::u4::new(0x1),
                channel: ux::u4::new(0xF),
                note: ux::u7::new(0x40),
                detach: true,
                reset: true,
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x6),
                channel: ux::u4::new(0x6),
                note: ux::u7::new(0x4F),
                detach: true,
                reset: true,
            }
            .to_ump(&mut [0x0]),
            &[0x46F6_4F03],
        );
    }
}
