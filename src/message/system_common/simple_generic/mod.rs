macro_rules! simple_generic_message {
    ($op_code:expr) => {
        use crate::{
            error::Error,
            message::Midi2Message,
            util::{builder, getter},
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Message {
            group: ux::u4,
        }

        builder::builder!(group: ux::u4);

        impl Message {
            const STATUS_CODE: u8 = $op_code;
            getter::getter!(group, ux::u4);
            builder::builder_method!();
        }

        impl Midi2Message for Message {
            fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
                crate::message::system_common::validate_packet(bytes, Message::STATUS_CODE)
            }
            fn from_ump(bytes: &[u32]) -> Self {
                Message {
                    group: crate::message::helpers::group_from_packet(bytes),
                }
            }
            fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32] {
                crate::message::system_common::write_data_to_packet(
                    bytes,
                    self.group,
                    Message::STATUS_CODE,
                    None,
                    None,
                );
                &bytes[..1]
            }
        }
    };
}

pub(crate) use simple_generic_message;

pub mod tune_request {
    use super::simple_generic_message;
    simple_generic_message!(0xF6);
}
pub mod timing_clock {
    use super::simple_generic_message;
    simple_generic_message!(0xF8);
}
pub mod start {
    use super::simple_generic_message;
    simple_generic_message!(0xFA);
}
pub mod cont {
    use super::simple_generic_message;
    simple_generic_message!(0xFB);
}
pub mod stop {
    use super::simple_generic_message;
    simple_generic_message!(0xFC);
}
pub mod active_sensing {
    use super::simple_generic_message;
    simple_generic_message!(0xFE);
}
pub mod reset {
    use super::simple_generic_message;
    simple_generic_message!(0xFF);
}

#[cfg(test)]
mod tests {
    use super::simple_generic_message;

    simple_generic_message!(0xFF);

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x2),
            }
            .to_ump(&mut [0x0]),
            &[0x12FF_0000],
        )
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x1CFF_0000]),
            Ok(Message {
                group: ux::u4::new(0xC)
            }),
        )
    }

    #[test]
    fn build() {
        assert_eq!(
            Message::builder().group(ux::u4::new(0xA)).build(),
            Message {
                group: ux::u4::new(0xA)
            },
        );
    }

    #[test]
    fn getters() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x5)
            }
            .group(),
            ux::u4::new(0x5),
        );
    }
}
