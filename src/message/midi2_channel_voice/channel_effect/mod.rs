use crate::{
    error::Error,
    packet::Packet,
};
use super::super::channel_voice_helpers;

pub mod pitch_bend;
pub mod pressure;

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message<const OP: u8> {
    group: ux::u4,
    channel: ux::u4,
    data: u32,
}

impl<const OP: u8> Message<OP> {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const OP_CODE: ux::u4 = ux::u4::new(OP);
}

impl<const OP: u8> std::convert::TryFrom<Packet> for Message<OP> {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        channel_voice_helpers::validate_packet(
            &p,
            Message::<OP>::TYPE_CODE,
            Message::<OP>::OP_CODE,
        )?;
        Ok(Message{
            group: channel_voice_helpers::group_from_packet(&p),
            channel: channel_voice_helpers::channel_from_packet(&p),
            data: p[1],
        })
    }
}

impl<const OP: u8> From<Message<OP>> for Packet {
    fn from(m: Message<OP>) -> Self {
        let mut p = Packet::new();
        channel_voice_helpers::write_data_to_packet(
            Message::<OP>::TYPE_CODE,
            m.group,
            Message::<OP>::OP_CODE,
            m.channel,
            &mut p,
        );
        p[1] = m.data;
        p
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        util::message_traits_test,
        packet::Packet,
    };

    type Message = super::Message<0b1111>;
    
    message_traits_test!(Message);
    
    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x6),
                channel: ux::u4::new(0x6),
                data: 0x8765_4321,
            }),
            Packet::from_data(&[0x46F6_0000, 0x8765_4321]),
        );
    }
    
    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x49F9_0000, 0x5101_5202])),
            Ok(Message {
                group: ux::u4::new(0x9),
                channel: ux::u4::new(0x9),
                data: 0x5101_5202,
            }),
        );
    }
}