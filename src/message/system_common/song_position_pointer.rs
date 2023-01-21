use super::super::helpers;
use crate::{
    error::Error,
    message::Midi2Message,
    util::{builder, getter, BitOps, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    position: ux::u14,
}

builder::builder!(group: ux::u4, position: ux::u14);

impl Message {
    const STATUS_CODE: u8 = 0xF2;
    getter::getter!(group, ux::u4);
    getter::getter!(position, ux::u14);
    builder::builder_method!();
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        super::validate_packet(bytes, Message::STATUS_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: helpers::group_from_packet(bytes),
            position: helpers::concatenate(
                bytes[0].octet(2).truncate(),
                bytes[0].octet(3).truncate(),
            ),
        }
    }
    fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32] {
        super::write_data_to_packet(
            bytes,
            self.group,
            Message::STATUS_CODE,
            Some(helpers::least_significant_bit(self.position)),
            Some(helpers::most_significant_bit(self.position)),
        );
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
            Message::try_from_ump(&[0x1FF2_0000 | 0b0101_1110_0011_0001]),
            Ok(Message {
                group: ux::u4::new(0xF),
                position: ux::u14::new(0b01_1000_1101_1110),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x1),
                position: ux::u14::new(0b00_1100_1011_1001)
            }
            .to_ump(&mut [0x0]),
            &[0x11F2_0000 | 0b0011_1001_0001_1001],
        );
    }
}
