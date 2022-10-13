use crate::{
    error::Error,
    util::Numeric, 
    packet::{Packet, PacketMethods},
};
use super::controllers;
use super::super::channel_voice_helpers;

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    note: ux::u7,
    controller: controllers::Controller,
}

impl Message {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(0b0000);
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        channel_voice_helpers::validate_packet(
            &p, 
            Message::TYPE_CODE, 
            Message::OP_CODE,
        )?;
        Ok(Message {
            group: channel_voice_helpers::group_from_packet(&p),
            channel: channel_voice_helpers::channel_from_packet(&p),
            note: p.octet(2).truncate(),
            controller: controllers::try_from_index_and_data(p.octet(3), p[1])?,
        })
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        channel_voice_helpers::write_data_to_packet(
            Message::TYPE_CODE, 
            m.group, 
            Message::OP_CODE, 
            m.channel, 
            &mut p
        );
        p.set_octet(2, m.note.into());
        let (index, data) = controllers::to_index_and_data(m.controller);
        p.set_octet(3, index);
        p[1] = data;
        p
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
            Message::try_from(Packet::from_data(&[
                0x4B06_7B03, 
                0b1011001_1001011101111110010111010,
            ])),
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
            Message::try_from(Packet::from_data(&[0x4000_0004, 0x0])),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x9),
                channel: ux::u4::new(0x7),
                note: ux::u7::new(0x30),
                controller: controllers::Controller::AttackTime(0x3141_5926),
            }),
            Packet::from_data(&[0x4907_3049, 0x31415926]),
        )
    }
}