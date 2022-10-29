pub mod pitch_bend;
pub mod pressure;

macro_rules! channel_effect_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            packet::Packet,
        };
        use crate::message::helpers;

        #[derive(
            Clone,
            Debug, 
            PartialEq, Eq,
        )]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            data: u32,
        }

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi2_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
        }

        impl core::convert::TryFrom<Packet> for Message {
            type Error = Error;
            fn try_from(p: Packet) -> Result<Self, Self::Error> {
                helpers::validate_packet(
                    &p,
                    Message::TYPE_CODE,
                    Message::OP_CODE,
                )?;
                Ok(Message{
                    group: helpers::group_from_packet(&p),
                    channel: helpers::channel_from_packet(&p),
                    data: p[1],
                })
            }
        }

        impl From<Message> for Packet {
            fn from(m: Message) -> Self {
                let mut p = Packet::new();
                helpers::write_data_to_packet(
                    Message::TYPE_CODE,
                    m.group,
                    Message::OP_CODE,
                    m.channel,
                    &mut p,
                );
                p[1] = m.data;
                p
            }
        }
    }
}

pub(crate) use channel_effect_message;

#[cfg(test)]
mod tests {
    use crate::util::message_traits_test;
    use super::channel_effect_message;

    channel_effect_message!(0b1111);
    
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