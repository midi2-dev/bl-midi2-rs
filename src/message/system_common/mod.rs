pub const UMP_MESSAGE_TYPE: u8 = 0x1;

pub mod song_position_pointer;
pub mod song_select;
pub mod time_code;

pub mod tune_request {
    use crate::message::{
        common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub const STATUS: u8 = 0xF6;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
    struct TuneRequest {
        #[property(crate::message::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::message::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::numeric_types::u4,
    }
}
pub mod timing_clock {
    use crate::message::{
        common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub const STATUS: u8 = 0xF8;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
    struct TimingClock {
        #[property(crate::message::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::message::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::numeric_types::u4,
    }
}
pub mod start {
    use crate::message::{
        common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub const STATUS: u8 = 0xFA;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
    struct Start {
        #[property(crate::message::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::message::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::numeric_types::u4,
    }
}
pub mod cont {
    use crate::message::{
        common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub const STATUS: u8 = 0xFB;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
    struct Continue {
        #[property(crate::message::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::message::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::numeric_types::u4,
    }
}
pub mod stop {
    use crate::message::{
        common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub const STATUS: u8 = 0xFC;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
    struct Stop {
        #[property(crate::message::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::message::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::numeric_types::u4,
    }
}
pub mod active_sensing {
    use crate::message::{
        common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub const STATUS: u8 = 0xFE;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
    struct ActiveSensing {
        #[property(crate::message::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::message::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::numeric_types::u4,
    }
}
pub mod reset {
    use crate::message::{
        common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub const STATUS: u8 = 0xFF;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
    struct Reset {
        #[property(crate::message::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::message::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::numeric_types::u4,
    }
}

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::Grouped,
    midi2_proc::FromBytes,
    midi2_proc::FromUmp,
    midi2_proc::TryFromBytes,
    midi2_proc::TryFromUmp,
    midi2_proc::RebufferFrom,
    midi2_proc::TryRebufferFrom,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[non_exhaustive]
pub enum SystemCommon<B: crate::buffer::Buffer> {
    ActiveSensing(active_sensing::ActiveSensing<B>),
    Continue(cont::Continue<B>),
    Reset(reset::Reset<B>),
    SongPositionPointer(song_position_pointer::SongPositionPointer<B>),
    SongSelect(song_select::SongSelect<B>),
    Start(start::Start<B>),
    Stop(stop::Stop<B>),
    TimeCode(time_code::TimeCode<B>),
    TimingClock(timing_clock::TimingClock<B>),
    TuneRequest(tune_request::TuneRequest<B>),
}

impl<'a, U: crate::buffer::Unit> core::convert::TryFrom<&'a [U]> for SystemCommon<&'a [U]> {
    type Error = crate::error::Error;
    fn try_from(buffer: &'a [U]) -> Result<Self, Self::Error> {
        if buffer.len() < 1 {
            return Err(crate::error::Error::InvalidData("Slice is too short"));
        };

        Ok(match status(&buffer) {
            active_sensing::STATUS => active_sensing::ActiveSensing::try_from(buffer)?.into(),
            cont::STATUS => cont::Continue::try_from(buffer)?.into(),
            reset::STATUS => reset::Reset::try_from(buffer)?.into(),
            song_position_pointer::STATUS => {
                song_position_pointer::SongPositionPointer::try_from(buffer)?.into()
            }
            song_select::STATUS => song_select::SongSelect::try_from(buffer)?.into(),
            start::STATUS => start::Start::try_from(buffer)?.into(),
            stop::STATUS => stop::Stop::try_from(buffer)?.into(),
            time_code::STATUS => time_code::TimeCode::try_from(buffer)?.into(),
            timing_clock::STATUS => timing_clock::TimingClock::try_from(buffer)?.into(),
            tune_request::STATUS => tune_request::TuneRequest::try_from(buffer)?.into(),
            _ => Err(crate::error::Error::InvalidData(
                "Unknown midi1 channel voice status",
            ))?,
        })
    }
}

struct SystemCommonStatus<const STATUS: u8>;

impl<const STATUS: u8, B: crate::buffer::Buffer> crate::util::property::Property<B>
    for SystemCommonStatus<STATUS>
{
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if status(buffer.buffer()) != STATUS {
            Err(crate::Error::InvalidData("Incorrect status field"))
        } else {
            Ok(())
        }
    }
    fn write(buffer: &mut B, _: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U32 => {
                use crate::buffer::{SpecialiseU32, UmpPrivateMut};
                use crate::util::BitOps;
                buffer.buffer_mut().specialise_u32_mut().message_mut()[0].set_octet(1, STATUS);
            }
            crate::buffer::UNIT_ID_U8 => {
                use crate::buffer::SpecialiseU8;
                buffer.buffer_mut().specialise_u8_mut()[0] = STATUS;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

fn status<U: crate::buffer::Unit>(buffer: &[U]) -> u8 {
    match <U as crate::buffer::UnitPrivate>::UNIT_ID {
        crate::buffer::UNIT_ID_U32 => {
            use crate::buffer::UmpPrivate;
            use crate::util::BitOps;
            <U as crate::buffer::UnitPrivate>::specialise_buffer_u32(buffer).message()[0].octet(1)
        }
        crate::buffer::UNIT_ID_U8 => {
            <U as crate::buffer::UnitPrivate>::specialise_buffer_u8(buffer)[0]
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_byte_data() {
        assert_eq!(
            SystemCommon::try_from(&[0xF3_u8, 0x4D][..]),
            song_select::SongSelect::try_from(&[0xF3_u8, 0x4D][..]).map(|m| m.into())
        );
    }

    #[test]
    fn from_ump_data() {
        assert_eq!(
            SystemCommon::try_from(&[0x15F1_5F00_u32][..]),
            time_code::TimeCode::try_from(&[0x15F1_5F00_u32][..]).map(|m| m.into())
        );
    }
}
