use crate::{
    error::Error,
    packet::Packet,
};
pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod utility;

pub trait Message: Sized {
    const TYPE: ux::u4;
    fn group(&self) -> ux::u4;
}

trait MessagePrivate : Message {
    fn validate_packet(p: &Packet) -> Result<(), Error> {
        if p.nibble(0) != <Self as Message>::TYPE {
            Err(Error::InvalidData)
        } else {
            Ok(())
        }
    }

    fn write_group_to_packet(g: ux::u4, p: &mut Packet) {
        p.set_nibble(0, <Self as Message>::TYPE);
        p.set_nibble(1, g);
    }
    
    fn group_from_packet(p: &Packet) -> ux::u4 {
        p.nibble(1)
    }
}

impl<T> MessagePrivate for T where T: Message {}