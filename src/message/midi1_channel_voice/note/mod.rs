pub mod note_off;
pub mod note_on;

macro_rules! note_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            message::helpers,
            packet::{Packet, PacketMethods},
            util::Truncate,
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            note: ux::u7,
            velocity: ux::u7,
        }

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi1_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
        }

        impl core::convert::TryFrom<Packet> for Message {
            type Error = Error;
            fn try_from(p: Packet) -> Result<Self, Self::Error> {
                helpers::validate_packet(&p, Message::TYPE_CODE, Message::OP_CODE)?;
                Ok(Message {
                    group: p.nibble(1),
                    channel: p.nibble(3),
                    note: p.octet(2).truncate(),
                    velocity: p.octet(3).truncate(),
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
                p.set_octet(2, m.note.into())
                    .set_octet(3, m.velocity.into());
                p
            }
        }
    };
}

pub(crate) use note_message;
