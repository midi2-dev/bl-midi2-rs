pub mod note_off;
pub mod note_on;

macro_rules! note_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            message::{
                helpers as message_helpers,
                midi2_channel_voice::{attribute, helpers},
                Midi2Message,
            },
            util::{builder, getter, BitOps, Truncate},
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
            channel: ux::u4,
            note: ux::u7,
            velocity: u16,
            attribute: Option<attribute::Attribute>,
        }

        #[derive(Clone, Default)]
        pub struct Builder {
            group: Option<ux::u4>,
            channel: Option<ux::u4>,
            note: Option<ux::u7>,
            velocity: Option<u16>,
            attribute: Option<attribute::Attribute>,
        }

        impl Builder {
            builder::builder_setter!(group: ux::u4);
            builder::builder_setter!(channel: ux::u4);
            builder::builder_setter!(note: ux::u7);
            builder::builder_setter!(velocity: u16);
            builder::builder_setter!(attribute: attribute::Attribute);

            pub fn build(&self) -> Message {
                Message {
                    group: self.group.unwrap_or_else(|| panic!("Missing fields!")),
                    channel: self.channel.unwrap_or_else(|| panic!("Missing fields!")),
                    note: self.note.unwrap_or_else(|| panic!("Missing fields!")),
                    velocity: self.velocity.unwrap_or_else(|| panic!("Missing fields!")),
                    attribute: self.attribute,
                }
            }
        }

        impl Message {
            const TYPE_CODE: ux::u4 = crate::message::midi2_channel_voice::TYPE_CODE;
            const OP_CODE: ux::u4 = ux::u4::new($op_code);
            getter::getter!(group, ux::u4);
            getter::getter!(channel, ux::u4);
            getter::getter!(note, ux::u7);
            getter::getter!(velocity, u16);
            getter::getter!(attribute, Option<attribute::Attribute>);
            builder::builder_method!();
        }

        impl Midi2Message for Message {
            fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
                attribute::validate_ump(bytes)?;
                helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
            }
            fn from_ump(bytes: &[u32]) -> Self {
                Message {
                    group: message_helpers::group_from_packet(bytes),
                    channel: message_helpers::channel_from_packet(bytes),
                    note: bytes[0].octet(2).truncate(),
                    velocity: bytes[1].word(0),
                    attribute: attribute::from_ump(bytes),
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
                bytes[1].set_word(0, self.velocity);
                attribute::write_attribute(bytes, self.attribute);
                &bytes[..2]
            }
        }
    };
}

pub(crate) use note_message;
