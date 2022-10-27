pub mod assignable;
pub mod registered;
pub mod relative_assignable;
pub mod relative_registered;

macro_rules! controller_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            util::Truncate, 
            packet::{Packet, PacketMethods},
        };
        use crate::message::helpers;

        #[derive(
            Clone,
            Debug, 
            PartialEq,
        )]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            bank: ux::u7,
            index: ux::u7,
            data: u32,
        }

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi2_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
        }

        impl std::convert::TryFrom<Packet> for Message {
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
                    bank: p.octet(2).truncate(),
                    index: p.octet(3).truncate(),
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
                p
                    .set_octet(2, m.bank.into())
                    .set_octet(3, m.index.into());
                p[1] = m.data;
                p
            }
        }
    }
}

pub(crate) use controller_message;

#[cfg(test)]
mod tests {
    use crate::util::message_traits_test;
    use super::controller_message;
    
    controller_message!(0b1111);
    
    message_traits_test!(Message);
    
    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x3),
                channel: ux::u4::new(0xC),
                bank: ux::u7::new(0x51),
                index: ux::u7::new(0x3F),
                data: 0xF00F_F00F,
            }),
            Packet::from_data(&[0x43FC_513F, 0xF00F_F00F]),
        );
    }
    
    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x4FFB_1011, 0xBABE_BABE])),
            Ok(Message {
                group: ux::u4::new(0xF),
                channel: ux::u4::new(0xB),
                bank: ux::u7::new(0x10),
                index: ux::u7::new(0x11),
                data: 0xBABE_BABE,
            })
        );
    }
}