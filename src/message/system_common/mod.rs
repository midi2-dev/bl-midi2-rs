use crate::{util::BitOps, *};

pub const TYPE_CODE: u32 = 0x1;

pub mod simple_generic;
pub mod song_position_pointer;
pub mod song_select;
pub mod time_code;

pub use simple_generic::active_sensing;
pub use simple_generic::cont;
pub use simple_generic::reset;
pub use simple_generic::start;
pub use simple_generic::stop;
pub use simple_generic::timing_clock;
pub use simple_generic::tune_request;

use active_sensing::ActiveSensingBorrowed;
use active_sensing::ActiveSensingBuilder;
use active_sensing::ActiveSensingMessage;
use active_sensing::ActiveSensingOwned;
use cont::ContinueBorrowed;
use cont::ContinueBuilder;
use cont::ContinueMessage;
use cont::ContinueOwned;
use reset::ResetBorrowed;
use reset::ResetBuilder;
use reset::ResetMessage;
use reset::ResetOwned;
use song_position_pointer::SongPositionPointerBorrowed;
use song_position_pointer::SongPositionPointerBuilder;
use song_position_pointer::SongPositionPointerMessage;
use song_position_pointer::SongPositionPointerOwned;
use song_select::SongSelectBorrowed;
use song_select::SongSelectBuilder;
use song_select::SongSelectMessage;
use song_select::SongSelectOwned;
use start::StartBorrowed;
use start::StartBuilder;
use start::StartMessage;
use start::StartOwned;
use stop::StopBorrowed;
use stop::StopBuilder;
use stop::StopMessage;
use stop::StopOwned;
use time_code::TimeCodeBorrowed;
use time_code::TimeCodeBuilder;
use time_code::TimeCodeMessage;
use time_code::TimeCodeOwned;
use timing_clock::TimingClockBorrowed;
use timing_clock::TimingClockBuilder;
use timing_clock::TimingClockMessage;
use timing_clock::TimingClockOwned;
use tune_request::TuneRequestBorrowed;
use tune_request::TuneRequestBuilder;
use tune_request::TuneRequestMessage;
use tune_request::TuneRequestOwned;

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
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

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
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

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
    pub fn builder() -> SystemCommonBuilder<Self> {
        SystemCommonBuilder::new()
    }
}

impl<'a> SystemCommonMessage<'a> {
    pub fn builder() -> SystemCommonBuilder<Self> {
        SystemCommonBuilder::new()
    }
}

impl<'a> core::convert::From<SystemCommonBorrowed<'a>> for SystemCommonMessage<'a> {
    fn from(value: SystemCommonBorrowed<'a>) -> Self {
        use SystemCommonBorrowed as B;
        use SystemCommonMessage as M;
        match value {
            B::ActiveSensing(m) => M::ActiveSensing(m.into()),
            B::Continue(m) => M::Continue(m.into()),
            B::Reset(m) => M::Reset(m.into()),
            B::SongPositionPointer(m) => M::SongPositionPointer(m.into()),
            B::SongSelect(m) => M::SongSelect(m.into()),
            B::Start(m) => M::Start(m.into()),
            B::Stop(m) => M::Stop(m.into()),
            B::TimeCode(m) => M::TimeCode(m.into()),
            B::TimingClock(m) => M::TimingClock(m.into()),
            B::TuneRequest(m) => M::TuneRequest(m.into()),
        }
    }
}

impl<'a> core::convert::From<SystemCommonOwned> for SystemCommonMessage<'a> {
    fn from(value: SystemCommonOwned) -> Self {
        use SystemCommonMessage as M;
        use SystemCommonOwned as O;
        match value {
            O::ActiveSensing(m) => M::ActiveSensing(m.into()),
            O::Continue(m) => M::Continue(m.into()),
            O::Reset(m) => M::Reset(m.into()),
            O::SongPositionPointer(m) => M::SongPositionPointer(m.into()),
            O::SongSelect(m) => M::SongSelect(m.into()),
            O::Start(m) => M::Start(m.into()),
            O::Stop(m) => M::Stop(m.into()),
            O::TimeCode(m) => M::TimeCode(m.into()),
            O::TimingClock(m) => M::TimingClock(m.into()),
            O::TuneRequest(m) => M::TuneRequest(m.into()),
        }
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

impl<'a> FromData<'a> for SystemCommonMessage<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        SystemCommonBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        SystemCommonBorrowed::from_data_unchecked(buffer).into()
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

impl<'a> ToOwned for SystemCommonMessage<'a> {
    type Owned = SystemCommonOwned;
    fn to_owned(self) -> Self::Owned {
        use SystemCommonMessage as M;
        use SystemCommonOwned as O;
        match self {
            M::ActiveSensing(m) => O::ActiveSensing(m.to_owned()),
            M::Continue(m) => O::Continue(m.to_owned()),
            M::Reset(m) => O::Reset(m.to_owned()),
            M::SongPositionPointer(m) => O::SongPositionPointer(m.to_owned()),
            M::SongSelect(m) => O::SongSelect(m.to_owned()),
            M::Start(m) => O::Start(m.to_owned()),
            M::Stop(m) => O::Stop(m.to_owned()),
            M::TimeCode(m) => O::TimeCode(m.to_owned()),
            M::TimingClock(m) => O::TimingClock(m.to_owned()),
            M::TuneRequest(m) => O::TuneRequest(m.to_owned()),
        }
    }
}

macro_rules! from_message_impl {
    ($message: ty) => {
        impl<'a> core::convert::From<$message> for SystemCommonMessage<'a> {
            fn from(value: $message) -> Self {
                <SystemCommonOwned as core::convert::From<$message>>::from(value).into()
            }
        }
    };
}

from_message_impl!(ActiveSensingOwned);
from_message_impl!(ContinueOwned);
from_message_impl!(ResetOwned);
from_message_impl!(SongPositionPointerOwned);
from_message_impl!(SongSelectOwned);
from_message_impl!(StartOwned);
from_message_impl!(StopOwned);
from_message_impl!(TimeCodeOwned);
from_message_impl!(TimingClockOwned);
from_message_impl!(TuneRequestOwned);

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
