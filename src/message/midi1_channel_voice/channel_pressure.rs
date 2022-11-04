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
    pressure: ux::u7,
}

builder::builder!(group: ux::u4, channel: ux::u4, pressure: ux::u7);

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1101);

    getter::getter!(group, ux::u4);
    getter::getter!(channel, ux::u4);
    getter::getter!(pressure, ux::u7);
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
    }

    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: bytes[0].nibble(1),
            channel: bytes[0].nibble(3),
            pressure: bytes[0].octet(2).truncate(),
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
        bytes[0].set_octet(2, self.pressure.into());
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
            Message::try_from_ump(&[0x24D4_5300]),
            Ok(Message {
                group: ux::u4::new(0x4),
                channel: ux::u4::new(4),
                pressure: ux::u7::new(83),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x5),
                channel: ux::u4::new(0xF),
                pressure: ux::u7::new(0x02),
            }
            .to_ump(&mut [0x0]),
            &[0x25DF_0200],
        );
    }
}
