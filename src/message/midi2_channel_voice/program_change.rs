use crate::{
    error::Error,
    message::{helpers as message_helpers, midi2_channel_voice::helpers, Midi2Message},
    util::{builder, getter, BitOps, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    channel: ux::u4,
    program: ux::u7,
    bank: Option<ux::u14>,
}

#[derive(Clone)]
pub struct Builder {
    group: Option<ux::u4>,
    channel: Option<ux::u4>,
    program: Option<ux::u7>,
    bank: Option<ux::u14>,
}

impl Builder {
    builder::builder_setter!(group: ux::u4);
    builder::builder_setter!(channel: ux::u4);
    builder::builder_setter!(program: ux::u7);
    builder::builder_setter!(bank: ux::u14);

    pub fn build(&self) -> Message {
        Message {
            group: self.group.unwrap_or_else(|| panic!("Missing fields!")),
            channel: self.channel.unwrap_or_else(|| panic!("Missing fields!")),
            program: self.program.unwrap_or_else(|| panic!("Missing fields!")),
            bank: self.bank,
        }
    }
}

impl Message {
    const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    const OP_CODE: ux::u4 = ux::u4::new(0b1100);

    pub fn builder() -> Builder {
        Builder {
            group: None,
            channel: None,
            program: None,
            bank: None,
        }
    }
    getter::getter!(group, ux::u4);
    getter::getter!(channel, ux::u4);
    getter::getter!(program, ux::u7);
    getter::getter!(bank, Option<ux::u14>);
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        helpers::validate_packet(bytes, Message::TYPE_CODE, Message::OP_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: message_helpers::group_from_packet(bytes),
            channel: message_helpers::channel_from_packet(bytes),
            program: bytes[1].octet(0).truncate(),
            bank: match bytes[0].octet(3) & 0b0000_0001 {
                1 => Some(ux::u14::from(bytes[1].octet(2)) << 7 | ux::u14::from(bytes[1].octet(3))),
                _ => None,
            },
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
        bytes[1].set_octet(0, self.program.into());
        if let Some(v) = self.bank {
            bytes[1].set_octet(2, (v >> 7).truncate());
            bytes[1].set_octet(3, v.truncate());
        }
        &bytes[..2]
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
            Message::try_from_ump(&[0x42C0_0001, 0x6600_7F7F,]),
            Ok(Message {
                group: ux::u4::new(0x2),
                channel: ux::u4::new(0x0),
                program: ux::u7::new(0x66),
                bank: Some(ux::u14::new(0b11_1111_1111_1111))
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x0),
                channel: ux::u4::new(0xD),
                program: ux::u7::new(0x7C),
                bank: None,
            }
            .to_ump(&mut [0x0, 0x0]),
            &[0x40CD_0000, 0x7C00_0000,],
        )
    }
}
