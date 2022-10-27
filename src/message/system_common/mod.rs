use crate::{
  error::Error,
  packet::{Packet, PacketMethods},
};

const TYPE_CODE: ux::u4 = ux::u4::new(0x1);

pub mod time_code;
pub mod song_position_pointer;

mod simple_generic;

pub use simple_generic::tune_request;
pub use simple_generic::timing_clock;
pub use simple_generic::start;
pub use simple_generic::cont;
pub use simple_generic::stop;
pub use simple_generic::active_sensing;
pub use simple_generic::reset;

fn validate_packet(p: &Packet, status: u8) -> Result<(), Error> {
    if p.nibble(0) != TYPE_CODE {
        Err(Error::InvalidData)
    } else if p.octet(1) != status {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn write_data_to_packet(
    p: &mut Packet,
    group: ux::u4, 
    status: u8, 
    byte1: Option<ux::u7>,
    byte2: Option<ux::u7>
) {
    super::write_type_to_packet(TYPE_CODE, p);
    super::write_group_to_packet(group, p);
    p.set_octet(1, status);
    if let Some(b) = byte1 {
        p.set_octet(2, b.into());
    }
    if let Some(b) = byte2 {
        p.set_octet(3, b.into());
    }
}