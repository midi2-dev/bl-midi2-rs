use crate::{
    error::Error,
    packet::Packet,
};

pub mod key_pressure;
pub mod assignable_per_note_controller;
pub mod registered_per_note_controller;

mod attribute;
mod controllers;
mod note;

pub use attribute::Attribute;
pub use controllers::Controller;
pub use note::note_on;
pub use note::note_off;

const TYPE_CODE: ux::u4 = ux::u4::new(0x4);

fn validate_packet(p: &Packet, op_code: ux::u4) -> Result<(), Error> {
    if p.nibble(0) != TYPE_CODE {
        Err(Error::InvalidData)
    } else if p.nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn write_data_to_packet(
    group: ux::u4,
    op_code: ux::u4,
    channel: ux::u4,
    p: &mut Packet,
) {
    super::write_type_to_packet(TYPE_CODE, p);
    p
        .set_nibble(1, group)
        .set_nibble(2, op_code)
        .set_nibble(3, channel);
}

fn group_from_packet(p: &Packet) -> ux::u4 {
    p.nibble(1)
}

fn channel_from_packet(p: &Packet) -> ux::u4 {
    p.nibble(3)
}