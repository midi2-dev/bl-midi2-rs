use crate::{
    error::Error,
    packet::{Packet, PacketMethods},
};

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

pub use active_sensing::Builder as ActiveSensingBuilder;
pub use active_sensing::Message as ActiveSensingMessage;
pub use cont::Builder as ContinueBuilder;
pub use cont::Message as ContinueMessage;
pub use reset::Builder as ResetBuilder;
pub use reset::Message as ResetMessage;
pub use song_position_pointer::Builder as SongPositionPointerBuilder;
pub use song_position_pointer::Message as SongPositionPointerMessage;
pub use song_select::Builder as SongSelectBuilder;
pub use song_select::Message as SongSelectMessage;
pub use start::Builder as StartBuilder;
pub use start::Message as StartMessage;
pub use stop::Builder as StopBuilder;
pub use stop::Message as StopMessage;
pub use time_code::Builder as TimeCodeBuilder;
pub use time_code::Message as TimeCodeMessage;
pub use timing_clock::Builder as TimingClockBuilder;
pub use timing_clock::Message as TimingClockMessage;
pub use tune_request::Builder as TuneRequestBuilder;
pub use tune_request::Message as TuneRequestMessage;

fn validate_packet(p: &Packet, status: u8) -> Result<(), Error> {
    if p.nibble(0) != TYPE_CODE || p.octet(1) != status {
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
    byte2: Option<ux::u7>,
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
