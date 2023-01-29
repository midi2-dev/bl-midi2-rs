use crate::{
    error::Error,
    message::Midi2Message,
    util::{builder, getter, BitOps},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
}

builder::builder!(group: ux::u4);

impl Message {
    const OP_CODE: ux::u4 = ux::u4::new(0x0);
    getter::getter!(group, ux::u4);
    builder::builder_method!();
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        super::validate_packet(bytes, Message::OP_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: bytes[0].nibble(1),
        }
    }
    fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32] {
        bytes[0].set_nibble(1, self.group);
        &bytes[..1]
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
            Message::try_from_ump(&[0x0700_0000]),
            Ok(Message {
                group: ux::u4::new(0x7)
            }),
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x2)
            }
            .to_ump(&mut [0x0]),
            &[0x0200_0000],
        );
    }
}
