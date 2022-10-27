pub mod key_pressure;
pub mod pitch_bend;

macro_rules! per_note_effect_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            util::Truncate,
            packet::{Packet, PacketMethods},
            message::helpers,
        };

        #[derive(
            Clone,
            Debug, 
            PartialEq,
        )]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            note: ux::u7,
            data: u32,
        }

        impl Message {
            pub const TYPE_CODE: ux::u4 = crate::message::midi2_channel_voice::TYPE_CODE;
            pub const OP_CODE: ux::u4 = ux::u4::new($op_code);
        }

        impl std::convert::TryFrom<Packet> for Message {
            type Error = Error;
            fn try_from(p: Packet) -> Result<Self, Self::Error> {
                helpers::validate_packet(
                    &p, 
                    Message::TYPE_CODE,
                    Message::OP_CODE,
                )?;
                Ok(Message {
                    group: helpers::group_from_packet(&p),
                    channel: helpers::channel_from_packet(&p),
                    note: p.octet(2).truncate(),
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
                    &mut p
                );
                p.set_octet(2, m.note.into());
                p[1] = m.data;
                p
            }
        }
    }
}

pub(crate) use per_note_effect_message;