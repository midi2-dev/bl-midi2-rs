use crate::{util::BitOps, *};

const TYPE_CODE: u32 = 0x1;

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

pub enum SystemCommonMessage<'a, B: Buffer> {
    ActiveSensing(ActiveSensingMessage<'a, B>),
    Continue(ContinueMessage<'a, B>),
    Reset(ResetMessage<'a, B>),
    SongPositionPointer(SongPositionPointerMessage<'a, B>),
    SongSelect(SongSelectMessage<'a, B>),
    Start(StartMessage<'a, B>),
    Stop(StopMessage<'a, B>),
    TimeCode(TimeCodeMessage<'a, B>),
    TimingClock(TimingClockMessage<'a, B>),
    TuneRequest(TuneRequestMessage<'a, B>),
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

impl<'a> Message<'a, Ump> for SystemCommonMessage<'a, Ump> {
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
            ACTIVE_SENSING => ActiveSensingMessage::<Ump>::validate_data(data),
            CONTINUE => ContinueMessage::<Ump>::validate_data(data),
            RESET => ResetMessage::<Ump>::validate_data(data),
            SONG_POSITION_POINTER => SongPositionPointerMessage::<Ump>::validate_data(data),
            SONG_SELECT => SongSelectMessage::<Ump>::validate_data(data),
            START => StartMessage::<Ump>::validate_data(data),
            STOP => StopMessage::<Ump>::validate_data(data),
            TIME_CODE => TimeCodeMessage::<Ump>::validate_data(data),
            TIMING_CLOCK => TimingClockMessage::<Ump>::validate_data(data),
            TUNE_REQUEST => TuneRequestMessage::<Ump>::validate_data(data),
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
