use crate::{error, util::BitOps};

mod no_op;
pub mod time_stamp;

pub use no_op::NoOpMessage;
pub use time_stamp::Builder as TimeStampMessageBuilder;
pub use time_stamp::Message as TimeStampMessage;

pub fn validate_packet(p: &[u32], op_code: ux::u4) -> Result<(), error::Error> {
    if p.is_empty() {
        Err(error::Error::BufferOverflow)
    } else if p[0].nibble(0) != ux::u4::new(0x0) || p[0].nibble(2) != op_code {
        Err(error::Error::InvalidData)
    } else {
        Ok(())
    }
}

pub fn validate_packet_2(p: &[u32], op_code: ux::u4) -> Result<(), error::InvalidData> {
    if p.is_empty() || p[0].nibble(0) != ux::u4::new(0x0) || p[0].nibble(2) != op_code {
        Err(error::InvalidData {})
    } else {
        Ok(())
    }
}
