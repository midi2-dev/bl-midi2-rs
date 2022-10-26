use crate::{
    packet::Packet,
    error::Error,
};

#[derive(
    Clone,
    Debug,
    PartialEq,
)]
pub struct Message<const OP: u8> {
    group: ux::u4,
}

impl<const OP: u8> Message<OP> {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const STATUS_CODE: u8 = OP;
}

impl<const OP: u8> std::convert::TryFrom<Packet> for Message<OP> {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        super::validate_type(&p, Message::<OP>::STATUS_CODE)?;
        Ok(Message::<OP> {
            group: super::super::helpers::group_from_packet(&p),
        })
    }
}

impl<const OP: u8> std::convert::From<Message<OP>> for Packet {
    fn from(m: Message<OP>) -> Self {
        let mut p = Packet::new();
        super::write_data_to_packet(
            &mut p,
            m.group,
            Message::<OP>::STATUS_CODE,
            None, 
            None
        );
        p
    }
}

pub mod tune_request { pub type Message = super::Message<0xF6>; }
pub mod timing_clock { pub type Message = super::Message<0xF8>; }
pub mod start { pub type Message = super::Message<0xFA>; }
pub mod cont { pub type Message = super::Message<0xFB>; }
pub mod stop { pub type Message = super::Message<0xFC>; }
pub mod active_sensing { pub type Message = super::Message<0xFE>; }
pub mod reset { pub type Message = super::Message<0xFF>; }

#[cfg(test)]
mod tests {
    use super::*;
    
    type GenericMessage = Message<0xFF>;
    
    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(GenericMessage {
                group: ux::u4::new(0x2),
            }),
            Packet::from_data(&[0x12FF_0000]),
        )
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            GenericMessage::try_from(Packet::from_data(&[0x1CFF_0000])),
            Ok(GenericMessage { group: ux::u4::new(0xC) }),
        )

    }
}