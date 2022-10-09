pub mod error;
// pub mod extended_system_exclusive;
pub mod message;
pub mod packet;
// pub mod system_common;
// pub mod system_exclusive;

mod bounded;
mod helpers;
mod slice_data;

#[cfg(test)]
mod builder_tests;
#[cfg(test)]
mod getter_tests;

pub trait Message {
    const TYPE: ux::u4;
    fn group(&self) -> ux::u4;
}

trait MessagePrivate : Message {
    fn validate_packet(p: &packet::Packet) -> Result<(), error::Error> {
        if p.nibble(0) != <Self as Message>::TYPE {
            Err(error::Error::InvalidData)
        } else {
            Ok(())
        }
    }

    fn write_data_to_packet(g: ux::u4, p: &mut packet::Packet) {
        p.set_nibble(0, <Self as Message>::TYPE);
        p.set_nibble(1, g);
    }
    
    fn group_from_packet(p: &packet::Packet) -> ux::u4 {
        p.nibble(1)
    }
}

impl<T> MessagePrivate for T where T: Message {}