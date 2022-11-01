pub mod note_off;
pub mod note_on;

macro_rules! note_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            message::{helpers, Midi2Message},
            util::{builder, getter, BitOps, Truncate},
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            note: ux::u7,
            velocity: ux::u7,
        }

        builder::builder!(
            group: ux::u4,
            channel: ux::u4,
            note: ux::u7,
            velocity: ux::u7
        );

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi1_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
            getter::getter!(group, ux::u4);
            getter::getter!(channel, ux::u4);
            getter::getter!(note, ux::u7);
            getter::getter!(velocity, ux::u7);
        }

        impl Midi2Message for Message {
            fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32] {
                helpers::write_data(
                    Message::TYPE_CODE,
                    self.group,
                    Message::OP_CODE,
                    self.channel,
                    bytes,
                );
                bytes[0]
                    .set_octet(2, self.note.into())
                    .set_octet(3, self.velocity.into());
                &bytes[0..1]
            }

            fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
                helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
            }

            fn from_ump(p: &[u32]) -> Message {
                Message {
                    group: p[0].nibble(1),
                    channel: p[0].nibble(3),
                    note: p[0].octet(2).truncate(),
                    velocity: p[0].octet(3).truncate(),
                }
            }
        }
    };
}

pub(crate) use note_message;
