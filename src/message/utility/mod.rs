use crate::{error::Error, util::BitOps};

pub mod no_op;
pub mod time_stamp;

pub use no_op::Builder as NoOpBuilder;
pub use no_op::Message as NoOpMessage;
pub use time_stamp::Builder as TimeStampBuilder;
pub use time_stamp::Message as TimeStampMessage;

pub fn validate_packet(p: &[u32], op_code: ux::u4) -> Result<(), Error> {
    if p.is_empty() {
        Err(Error::BufferOverflow)
    } else if p[0].nibble(0) != ux::u4::new(0x0) || p[0].nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
