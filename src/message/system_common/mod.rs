use crate::{util::BitOps, *};

pub const TYPE_CODE: u32 = 0x1;

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

pub use active_sensing::ActiveSensing;
pub use active_sensing::ActiveSensingBorrowed;
pub use active_sensing::ActiveSensingBuilder;
pub use active_sensing::ActiveSensingOwned;
pub use cont::Continue;
pub use cont::ContinueBorrowed;
pub use cont::ContinueBuilder;
pub use cont::ContinueOwned;
pub use reset::Reset;
pub use reset::ResetBorrowed;
pub use reset::ResetBuilder;
pub use reset::ResetOwned;
pub use song_position_pointer::SongPositionPointer;
pub use song_position_pointer::SongPositionPointerBorrowed;
pub use song_position_pointer::SongPositionPointerBuilder;
pub use song_position_pointer::SongPositionPointerOwned;
pub use song_select::SongSelect;
pub use song_select::SongSelectBorrowed;
pub use song_select::SongSelectBuilder;
pub use song_select::SongSelectOwned;
pub use start::Start;
pub use start::StartBorrowed;
pub use start::StartBuilder;
pub use start::StartOwned;
pub use stop::Stop;
pub use stop::StopBorrowed;
pub use stop::StopBuilder;
pub use stop::StopOwned;
pub use time_code::TimeCode;
pub use time_code::TimeCodeBorrowed;
pub use time_code::TimeCodeBuilder;
pub use time_code::TimeCodeOwned;
pub use timing_clock::TimingClock;
pub use timing_clock::TimingClockBorrowed;
pub use timing_clock::TimingClockBuilder;
pub use timing_clock::TimingClockOwned;
pub use tune_request::TuneRequest;
pub use tune_request::TuneRequestBorrowed;
pub use tune_request::TuneRequestBuilder;
pub use tune_request::TuneRequestOwned;

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
pub enum SystemCommonBorrowed<'a> {
    ActiveSensing(ActiveSensingBorrowed<'a>),
    Continue(ContinueBorrowed<'a>),
    Reset(ResetBorrowed<'a>),
    SongPositionPointer(SongPositionPointerBorrowed<'a>),
    SongSelect(SongSelectBorrowed<'a>),
    Start(StartBorrowed<'a>),
    Stop(StopBorrowed<'a>),
    TimeCode(TimeCodeBorrowed<'a>),
    TimingClock(TimingClockBorrowed<'a>),
    TuneRequest(TuneRequestBorrowed<'a>),
}

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
pub enum SystemCommonOwned {
    ActiveSensing(ActiveSensingOwned),
    Continue(ContinueOwned),
    Reset(ResetOwned),
    SongPositionPointer(SongPositionPointerOwned),
    SongSelect(SongSelectOwned),
    Start(StartOwned),
    Stop(StopOwned),
    TimeCode(TimeCodeOwned),
    TimingClock(TimingClockOwned),
    TuneRequest(TuneRequestOwned),
}

#[derive(Default)]
pub struct SystemCommonBuilder<M>(core::marker::PhantomData<M>)
where
    M: core::convert::From<ActiveSensingOwned>
        + core::convert::From<ContinueOwned>
        + core::convert::From<ResetOwned>
        + core::convert::From<SongPositionPointerOwned>
        + core::convert::From<SongSelectOwned>
        + core::convert::From<StartOwned>
        + core::convert::From<StopOwned>
        + core::convert::From<TimeCodeOwned>
        + core::convert::From<TimingClockOwned>
        + core::convert::From<TuneRequestOwned>;

impl<M> SystemCommonBuilder<M>
where
    M: core::convert::From<ActiveSensingOwned>
        + core::convert::From<ContinueOwned>
        + core::convert::From<ResetOwned>
        + core::convert::From<SongPositionPointerOwned>
        + core::convert::From<SongSelectOwned>
        + core::convert::From<StartOwned>
        + core::convert::From<StopOwned>
        + core::convert::From<TimeCodeOwned>
        + core::convert::From<TimingClockOwned>
        + core::convert::From<TuneRequestOwned>,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn active_sensing(self) -> ActiveSensingBuilder<M> {
        ActiveSensingBuilder::new()
    }
    pub fn cont(self) -> ContinueBuilder<M> {
        ContinueBuilder::new()
    }
    pub fn reset(self) -> ResetBuilder<M> {
        ResetBuilder::new()
    }
    pub fn song_position_pointer(self) -> SongPositionPointerBuilder<M> {
        SongPositionPointerBuilder::new()
    }
    pub fn song_select(self) -> SongSelectBuilder<M> {
        SongSelectBuilder::new()
    }
    pub fn start(self) -> StartBuilder<M> {
        StartBuilder::new()
    }
    pub fn stop(self) -> StopBuilder<M> {
        StopBuilder::new()
    }
    pub fn time_code(self) -> TimeCodeBuilder<M> {
        TimeCodeBuilder::new()
    }
    pub fn timing_clock(self) -> TimingClockBuilder<M> {
        TimingClockBuilder::new()
    }
    pub fn tune_request(self) -> TuneRequestBuilder<M> {
        TuneRequestBuilder::new()
    }
}

impl SystemCommonOwned {
    pub fn builder() -> SystemCommonBuilder<SystemCommonOwned> {
        SystemCommonBuilder::new()
    }
}

pub const ACTIVE_SENSING: u32 = 0xFE;
pub const CONTINUE: u32 = 0xFB;
pub const RESET: u32 = 0xFF;
pub const SONG_POSITION_POINTER: u32 = 0xF2;
pub const SONG_SELECT: u32 = 0xF3;
pub const START: u32 = 0xFA;
pub const STOP: u32 = 0xFC;
pub const TIME_CODE: u32 = 0xF1;
pub const TIMING_CLOCK: u32 = 0xF8;
pub const TUNE_REQUEST: u32 = 0xF6;

impl<'a> Data for SystemCommonBorrowed<'a> {
    fn data(&self) -> &[u32] {
        use SystemCommonBorrowed::*;
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
}

impl Data for SystemCommonOwned {
    fn data(&self) -> &[u32] {
        use SystemCommonOwned::*;
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
}

impl<'a> Grouped for SystemCommonBorrowed<'a> {
    fn group(&self) -> u4 {
        use SystemCommonBorrowed::*;
        match self {
            ActiveSensing(m) => m.group(),
            Continue(m) => m.group(),
            Reset(m) => m.group(),
            SongPositionPointer(m) => m.group(),
            SongSelect(m) => m.group(),
            Start(m) => m.group(),
            Stop(m) => m.group(),
            TimeCode(m) => m.group(),
            TimingClock(m) => m.group(),
            TuneRequest(m) => m.group(),
        }
    }
}

impl Grouped for SystemCommonOwned {
    fn group(&self) -> u4 {
        use SystemCommonOwned::*;
        match self {
            ActiveSensing(m) => m.group(),
            Continue(m) => m.group(),
            Reset(m) => m.group(),
            SongPositionPointer(m) => m.group(),
            SongSelect(m) => m.group(),
            Start(m) => m.group(),
            Stop(m) => m.group(),
            TimeCode(m) => m.group(),
            TimingClock(m) => m.group(),
            TuneRequest(m) => m.group(),
        }
    }
}

impl<'a> FromData<'a> for SystemCommonBorrowed<'a> {
    type Target = Self;
    fn validate_data(data: &[u32]) -> Result<()> {
        match data[0].octet(1).into() {
            ACTIVE_SENSING => ActiveSensingBorrowed::validate_data(data),
            CONTINUE => ContinueBorrowed::validate_data(data),
            RESET => ResetBorrowed::validate_data(data),
            SONG_POSITION_POINTER => SongPositionPointerBorrowed::validate_data(data),
            SONG_SELECT => SongSelectBorrowed::validate_data(data),
            START => StartBorrowed::validate_data(data),
            STOP => StopBorrowed::validate_data(data),
            TIME_CODE => TimeCodeBorrowed::validate_data(data),
            TIMING_CLOCK => TimingClockBorrowed::validate_data(data),
            TUNE_REQUEST => TuneRequestBorrowed::validate_data(data),
            _ => Err(Error::InvalidData),
        }
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        use SystemCommonBorrowed::*;
        match data[0].octet(1).into() {
            ACTIVE_SENSING => ActiveSensing(ActiveSensingBorrowed::from_data_unchecked(data)),
            CONTINUE => Continue(ContinueBorrowed::from_data_unchecked(data)),
            RESET => Reset(ResetBorrowed::from_data_unchecked(data)),
            SONG_POSITION_POINTER => {
                SongPositionPointer(SongPositionPointerBorrowed::from_data_unchecked(data))
            }
            SONG_SELECT => SongSelect(SongSelectBorrowed::from_data_unchecked(data)),
            START => Start(StartBorrowed::from_data_unchecked(data)),
            STOP => Stop(StopBorrowed::from_data_unchecked(data)),
            TIME_CODE => TimeCode(TimeCodeBorrowed::from_data_unchecked(data)),
            TIMING_CLOCK => TimingClock(TimingClockBorrowed::from_data_unchecked(data)),
            TUNE_REQUEST => TuneRequest(TuneRequestBorrowed::from_data_unchecked(data)),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            SystemCommonOwned::builder()
                .song_select()
                .song(u7::new(0x1))
                .build(),
            Ok(SystemCommonOwned::SongSelect(
                SongSelectOwned::builder()
                    .song(u7::new(0x1))
                    .build()
                    .unwrap()
            )),
        );
    }
}

impl<'a> ToOwned for SystemCommonBorrowed<'a> {
    type Owned = SystemCommonOwned;
    fn to_owned(self) -> Self::Owned {
        use SystemCommonBorrowed as B;
        use SystemCommonOwned as O;
        match self {
            B::ActiveSensing(m) => O::ActiveSensing(m.to_owned()),
            B::Continue(m) => O::Continue(m.to_owned()),
            B::Reset(m) => O::Reset(m.to_owned()),
            B::SongPositionPointer(m) => O::SongPositionPointer(m.to_owned()),
            B::SongSelect(m) => O::SongSelect(m.to_owned()),
            B::Start(m) => O::Start(m.to_owned()),
            B::Stop(m) => O::Stop(m.to_owned()),
            B::TimeCode(m) => O::TimeCode(m.to_owned()),
            B::TimingClock(m) => O::TimingClock(m.to_owned()),
            B::TuneRequest(m) => O::TuneRequest(m.to_owned()),
        }
    }
}
