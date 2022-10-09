use crate::{
    error::Error,
    packet::Packet,
};

pub mod key_pressure;
pub mod note_off;
pub mod note_on;

mod attribute;
pub use attribute::Attribute;

pub trait Message : super::Message {
    const OP_CODE: ux::u4;
    fn channel(&self) -> ux::u4;
}

struct Data {
    group: ux::u4,
    channel: ux::u4,
}

trait MessagePrivate : Message {
    fn validate_packet(p: &Packet) -> Result<(), Error> {
        <Self as super::MessagePrivate>::validate_packet(p)?;
        if p.nibble(2) != <Self as Message>::OP_CODE {
            Err(Error::InvalidData)
        } else {
            Ok(())
        }
    }
    
    fn write_data_to_packet(d: Data, p: &mut Packet) {
        <Self as super::MessagePrivate>::write_group_to_packet(d.group, p);
        p.set_nibble(2, <Self as Message>::OP_CODE);
        p.set_nibble(3, d.channel);
    }
    
    fn data_from_packet(p: &Packet) -> Data {
        Data {
            group: p.nibble(1),
            channel: p.nibble(3),
        }
    }
}

impl<T> MessagePrivate for T where T: Message {}