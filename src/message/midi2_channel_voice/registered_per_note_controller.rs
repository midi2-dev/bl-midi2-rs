use crate::{
    error::Error,
    message::{
        helpers as message_helpers,
        midi2_channel_voice::{controllers, helpers},
        Midi2Message,
    },
    util::{builder, getter, BitOps, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    controller: controllers::Controller,
}

builder::builder!(
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    controller: controllers::Controller
);

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b0000);
    getter::getter!(group, ux::u4);
    getter::getter!(channel, ux::u4);
    getter::getter!(note, ux::u7);
    getter::getter!(controller, controllers::Controller);
    builder::builder_method!();
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        controllers::validate_index(bytes[0].octet(3))?;
        helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: message_helpers::group_from_packet(bytes),
            channel: message_helpers::channel_from_packet(bytes),
            note: bytes[0].octet(2).truncate(),
            controller: controllers::from_index_and_data(bytes[0].octet(3), bytes[1]),
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
        let (index, data) = controllers::to_index_and_data(self.controller);
        bytes[0].set_octet(3, index);
        bytes[1] = data;
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
            Message::try_from_ump(&[0x4B06_7B03, 0b1011_0011_0010_1110_1111_1100_1011_1010,]),
            Ok(Message {
                group: ux::u4::new(0xB),
                channel: ux::u4::new(0x6),
                note: ux::u7::new(0x7B),
                controller: controllers::Controller::Pitch7_25 {
                    note: ux::u7::new(0b1011001),
                    pitch_up: ux::u25::new(0b1001011101111110010111010),
                }
            })
        );
    }

    #[test]
    fn deserialize_invalid_controller() {
        assert_eq!(
            Message::try_from_ump(&[0x4000_0004, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x9),
                channel: ux::u4::new(0x7),
                note: ux::u7::new(0x30),
                controller: controllers::Controller::AttackTime(0x3141_5926),
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x4907_3049, 0x31415926],
        )
    }
}
