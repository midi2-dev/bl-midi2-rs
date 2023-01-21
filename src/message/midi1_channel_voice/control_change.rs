use super::super::helpers;
use crate::{
    error::Error,
    message::Midi2Message,
    util::{builder, getter, BitOps, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    controller: ux::u7,
    value: ux::u7,
}

builder::builder!(
    group: ux::u4,
    channel: ux::u4,
    controller: ux::u7,
    value: ux::u7
);

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1011);
    getter::getter!(group, ux::u4);
    getter::getter!(channel, ux::u4);
    getter::getter!(controller, ux::u7);
    getter::getter!(value, ux::u7);
    builder::builder_method!();
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: bytes[0].nibble(1),
            channel: bytes[0].nibble(3),
            controller: bytes[0].octet(2).truncate(),
            value: bytes[0].octet(3).truncate(),
        }
    }
    fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32] {
        helpers::write_data(
            Message::TYPE_CODE,
            self.group,
            Message::OP_CODE,
            self.channel,
            bytes,
        );
        bytes[0]
            .set_octet(2, self.controller.into())
            .set_octet(3, self.value.into());
        &bytes[..1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::message_traits_test;

    message_traits_test!(Message);

    #[test]
    fn wrong_status() {
        assert_eq!(
            Message::try_from_ump(&[0x2000_0000]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x21BF_010A]),
            Ok(Message {
                group: ux::u4::new(0x1),
                channel: ux::u4::new(15),
                controller: ux::u7::new(1),
                value: ux::u7::new(10),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x9),
                channel: ux::u4::new(0x0),
                controller: ux::u7::new(0x30),
                value: ux::u7::new(0x32),
            }
            .to_ump(&mut [0x0]),
            &[0x29B0_3032],
        );
    }
}
