macro_rules! simple_generic_message {
    ($op_code:expr) => {
        use crate::{
            packet::Packet,
            error::Error,
        };

        #[derive(
            Clone,
            Debug,
            PartialEq, Eq,
        )]
        pub struct Message {
            group: ux::u4,
        }

        impl Message {
            const STATUS_CODE: u8 = $op_code;
        }

        impl core::convert::TryFrom<Packet> for Message {
            type Error = Error;
            fn try_from(p: Packet) -> Result<Self, Self::Error> {
                crate::message::system_common::validate_packet(&p, Message::STATUS_CODE)?;
                Ok(Message {
                    group: crate::message::helpers::group_from_packet(&p),
                })
            }
        }

        impl core::convert::From<Message> for Packet {
            fn from(m: Message) -> Self {
                let mut p = Packet::new();
                crate::message::system_common::write_data_to_packet(
                    &mut p,
                    m.group,
                    Message::STATUS_CODE,
                    None, 
                    None
                );
                p
            }
        }

    }
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
            Packet::from(Message {
                group: ux::u4::new(0x2),
            }),
            Packet::from_data(&[0x12FF_0000]),
        )
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x1CFF_0000])),
            Ok(Message { group: ux::u4::new(0xC) }),
        )

    }
}