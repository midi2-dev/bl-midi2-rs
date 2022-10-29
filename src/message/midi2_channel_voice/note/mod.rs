pub mod note_off;
pub mod note_on;

macro_rules! note_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            message::{helpers, midi2_channel_voice::attribute},
            packet::{Packet, PacketMethods},
            util::Truncate,
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            note: ux::u7,
            velocity: u16,
            attribute: Option<attribute::Attribute>,
        }

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi2_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
        }

        impl core::convert::TryFrom<Packet> for Message {
            type Error = Error;
            fn try_from(p: Packet) -> Result<Self, Self::Error> {
                helpers::validate_packet(&p, Message::TYPE_CODE, Message::OP_CODE)?;
                Ok(Message {
                    group: helpers::group_from_packet(&p),
                    channel: helpers::channel_from_packet(&p),
                    note: p.octet(2).truncate(),
                    velocity: p.word(2),
                    attribute: attribute::from_packet(&p)?,
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
                p.set_octet(2, m.note.into()).set_word(2, m.velocity);
                attribute::write_attribute(&mut p, m.attribute);
                p
            }
        }
    };
}

pub(crate) use note_message;
