pub mod key_pressure;
pub mod pitch_bend;

macro_rules! per_note_effect_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            message::{helpers as message_helpers, midi2_channel_voice::helpers, Midi2Message},
            util::{builder, getter, BitOps, Truncate},
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            note: ux::u7,
            data: u32,
        }

        builder::builder!(group: ux::u4, channel: ux::u4, note: ux::u7, data: u32);

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi2_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
            getter::getter!(group, ux::u4);
            getter::getter!(channel, ux::u4);
            getter::getter!(note, ux::u7);
            getter::getter!(data, u32);
        }

        impl Midi2Message for Message {
            fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
                helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
            }
            fn from_ump(bytes: &[u32]) -> Self {
                Message {
                    group: message_helpers::group_from_packet(bytes),
                    channel: message_helpers::channel_from_packet(bytes),
                    note: bytes[0].octet(2).truncate(),
                    data: bytes[1],
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
                bytes[1] = self.data;
                &bytes[..2]
            }
        }
    };
}

pub(crate) use per_note_effect_message;
