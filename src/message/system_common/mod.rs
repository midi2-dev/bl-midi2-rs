use crate::{util::BitOps, *};

const TYPE_CODE: u4 = u4::new(0x1);

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
pub use song_select::SongSelectBuilder;
pub use song_select::SongSelectMessage;
pub use start::StartBuilder;
pub use start::StartMessage;
pub use stop::StopBuilder;
pub use stop::StopMessage;
pub use time_code::TimeCodeBuilder;
pub use time_code::TimeCodeMessage;
pub use timing_clock::TimingClockBuilder;
pub use timing_clock::TimingClockMessage;
pub use tune_request::TuneRequestBuilder;
pub use tune_request::TuneRequestMessage;

pub enum SystemCommonMessage<'a> {
    ActiveSensing(ActiveSensingMessage<'a>),
    Continue(ContinueMessage<'a>),
    Reset(ResetMessage<'a>),
    SongPositionPointer(SongPositionPointerMessage<'a>),
    SongSelect(SongSelectMessage<'a>),
    Start(StartMessage<'a>),
    Stop(StopMessage<'a>),
    TimeCode(TimeCodeMessage<'a>),
    TimingClock(TimingClockMessage<'a>),
    TuneRequest(TuneRequestMessage<'a>),
}

use SystemCommonMessage::*;

const ACTIVE_SENSING: u8 = 0xFE;
const CONTINUE: u8 = 0xFB;
const RESET: u8 = 0xFF;
const SONG_POSITION_POINTER: u8 = 0xF2;
const SONG_SELECT: u8 = 0xF3;
const START: u8 = 0xFA;
const STOP: u8 = 0xFC;
const TIME_CODE: u8 = 0xF1;
const TIMING_CLOCK: u8 = 0xF8;
const TUNE_REQUEST: u8 = 0xF6;

impl<'a> Message<'a> for SystemCommonMessage<'a> {
    fn data(&self) -> &'a [u32] {
        match self {
            ActiveSensing(m) => m.data(),
            Continue(m) => m.data(),
            Reset(m) => m.data(),
            SongPositionPointer(m) => m.data(),
            SongSelect(m) => m.data(),
            Start(m) => m.data(),
            Stop(m) => m.data(),
            TimeCode(m) => m.data(),
            TimingClock(m) => m.data(),
            TuneRequest(m) => m.data(),
        }
    }
    fn validate_data(data: &[u32]) -> Result<()> {
        match data[0].octet(1) {
            ACTIVE_SENSING => ActiveSensingMessage::validate_data(data),
            CONTINUE => ContinueMessage::validate_data(data),
            RESET => ResetMessage::validate_data(data),
            SONG_POSITION_POINTER => SongPositionPointerMessage::validate_data(data),
            SONG_SELECT => SongSelectMessage::validate_data(data),
            START => StartMessage::validate_data(data),
            STOP => StopMessage::validate_data(data),
            TIME_CODE => TimeCodeMessage::validate_data(data),
            TIMING_CLOCK => TimingClockMessage::validate_data(data),
            TUNE_REQUEST => TuneRequestMessage::validate_data(data),
            _ => Err(Error::InvalidData),
        }
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        match data[0].octet(1) {
            ACTIVE_SENSING => ActiveSensing(ActiveSensingMessage::from_data_unchecked(data)),
            CONTINUE => Continue(ContinueMessage::from_data_unchecked(data)),
            RESET => Reset(ResetMessage::from_data_unchecked(data)),
            SONG_POSITION_POINTER => {
                SongPositionPointer(SongPositionPointerMessage::from_data_unchecked(data))
            }
            SONG_SELECT => SongSelect(SongSelectMessage::from_data_unchecked(data)),
            START => Start(StartMessage::from_data_unchecked(data)),
            STOP => Stop(StopMessage::from_data_unchecked(data)),
            TIME_CODE => TimeCode(TimeCodeMessage::from_data_unchecked(data)),
            TIMING_CLOCK => TimingClock(TimingClockMessage::from_data_unchecked(data)),
            TUNE_REQUEST => TuneRequest(TuneRequestMessage::from_data_unchecked(data)),
            _ => panic!(),
        }
    }
}

fn validate_packet(p: &[u32], status: u8) -> Result<()> {
    if p.is_empty() {
        Err(Error::BufferOverflow)
    } else if p[0].nibble(0) != TYPE_CODE || p[0].octet(1) != status {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn validate_buffer_size(buffer: &[u32]) -> Result<()> {
    if buffer.is_empty() {
        Err(Error::BufferOverflow)
    } else {
        Ok(())
    }
}

fn write_op_code_to_packet(buffer: &mut [u32], op_code: u8) {
    buffer[0].set_octet(1, op_code);
}
