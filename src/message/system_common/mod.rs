use crate::{error::Error, util::BitOps};

const TYPE_CODE: ux::u4 = ux::u4::new(0x1);

mod simple_generic;
mod song_position_pointer;
mod song_select;
mod time_code;

use simple_generic::active_sensing;
use simple_generic::cont;
use simple_generic::reset;
use simple_generic::start;
use simple_generic::stop;
use simple_generic::timing_clock;
use simple_generic::tune_request;

pub use active_sensing::ActiveSensingBuilder;
pub use active_sensing::ActiveSensingMessage;
pub use cont::ContinueBuilder;
pub use cont::ContinueMessage;
pub use reset::ResetBuilder;
pub use reset::ResetMessage;
pub use song_position_pointer::SongPositionPointerBuilder;
pub use song_position_pointer::SongPositionPointerMessage;
pub use song_select::Builder as SongSelectMessageBuilder;
pub use song_select::Message as SongSelectMessage;
pub use start::StartBuilder;
pub use start::StartMessage;
pub use stop::StopBuilder;
pub use stop::StopMessage;
pub use time_code::Builder as TimeCodeMessageBuilder;
pub use time_code::Message as TimeCodeMessage;
pub use timing_clock::TimingClockBuilder;
pub use timing_clock::TimingClockMessage;
pub use tune_request::TuneRequestBuilder;
pub use tune_request::TuneRequestMessage;

fn validate_packet(p: &[u32], status: u8) -> Result<(), Error> {
    if p.is_empty() {
        Err(Error::BufferOverflow)
    } else if p[0].nibble(0) != TYPE_CODE || p[0].octet(1) != status {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn validate_buffer_size(buffer: &[u32]) -> Result<(), Error> {
    if buffer.len() > 0 {
        Ok(())
    } else {
        Err(Error::BufferOverflow)
    }
}

fn write_op_code_to_packet(buffer: &mut [u32], op_code: u8) {
    buffer[0].set_octet(1, op_code);
}

fn write_data_to_packet(
    p: &mut [u32],
    group: ux::u4,
    status: u8,
    byte1: Option<ux::u7>,
    byte2: Option<ux::u7>,
) {
    super::helpers::write_type_to_packet(TYPE_CODE, p);
    super::helpers::write_group_to_packet(group, p);
    p[0].set_octet(1, status);
    if let Some(b) = byte1 {
        p[0].set_octet(2, b.into());
    }
    if let Some(b) = byte2 {
        p[0].set_octet(3, b.into());
    }
}
