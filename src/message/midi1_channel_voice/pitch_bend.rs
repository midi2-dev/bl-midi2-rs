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
    bend: ux::u14,
}

builder::builder!(group: ux::u4, channel: ux::u4, bend: ux::u14);

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1110);
    getter::getter!(group, ux::u4);
    getter::getter!(channel, ux::u4);
    getter::getter!(bend, ux::u14);
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: bytes[0].nibble(1),
            channel: bytes[0].nibble(3),
            bend: helpers::concatenate(bytes[0].octet(2).truncate(), bytes[0].octet(3).truncate()),
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
            .set_octet(2, helpers::least_significant_bit(self.bend).into())
            .set_octet(3, helpers::most_significant_bit(self.bend).into());
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
            Message::try_from_ump(&[0b0010_1011_1110_0000_0110_1001_0011_0011]),
            Ok(Message {
                group: ux::u4::new(0xB),
                channel: ux::u4::new(0),
                bend: ux::u14::new(0b01_1001_1110_1001),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x5),
                channel: ux::u4::new(0x0),
                bend: ux::u14::new(0b00_1101_1011_1001),
            }
            .to_ump(&mut [0x0]),
            &[0b0010_0101_1110_0000_0011_1001_0001_1011],
        );
    }
}
