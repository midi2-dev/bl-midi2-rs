#![doc = include_str!("system_common/README.md")]

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x1;

mod packet;
mod song_position_pointer;
mod song_select;
mod time_code;

mod tune_request {
    use crate::{
        detail::common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub(crate) const STATUS: u8 = 0xF6;
    /// Tune Request Message
    ///
    /// See the [module docs](crate::system_common) for more info.
    #[midi2_proc::generate_message(
        Via(system_common::SystemCommon),
        FixedSize,
        MinSizeUmp(1),
        MinSizeBytes(2)
    )]
    struct TuneRequest {
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::ux::u4,
    }
}
mod timing_clock {
    use crate::{
        detail::common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub(crate) const STATUS: u8 = 0xF8;
    /// Timing Clock Message
    ///
    /// See the [module docs](crate::system_common) for more info.
    #[midi2_proc::generate_message(
        Via(system_common::SystemCommon),
        FixedSize,
        MinSizeUmp(1),
        MinSizeBytes(1)
    )]
    struct TimingClock {
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::ux::u4,
    }
}
mod start {
    use crate::{
        detail::common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub(crate) const STATUS: u8 = 0xFA;
    /// Start Message
    ///
    /// See the [module docs](crate::system_common) for more info.
    #[midi2_proc::generate_message(
        Via(system_common::SystemCommon),
        FixedSize,
        MinSizeUmp(1),
        MinSizeBytes(1)
    )]
    struct Start {
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::ux::u4,
    }
}
mod cont {
    use crate::{
        detail::common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub(crate) const STATUS: u8 = 0xFB;
    /// Continue Message
    ///
    /// See the [module docs](crate::system_common) for more info.
    #[midi2_proc::generate_message(
        Via(system_common::SystemCommon),
        FixedSize,
        MinSizeUmp(1),
        MinSizeBytes(1)
    )]
    struct Continue {
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::ux::u4,
    }
}
mod stop {
    use crate::{
        detail::common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub(crate) const STATUS: u8 = 0xFC;
    /// Stop Message
    ///
    /// See the [module docs](crate::system_common) for more info.
    #[midi2_proc::generate_message(
        Via(system_common::SystemCommon),
        FixedSize,
        MinSizeUmp(1),
        MinSizeBytes(1)
    )]
    struct Stop {
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::ux::u4,
    }
}
mod active_sensing {
    use crate::{
        detail::common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub(crate) const STATUS: u8 = 0xFE;
    /// Active Sensing Message
    ///
    /// See the [module docs](crate::system_common) for more info.
    #[midi2_proc::generate_message(
        Via(system_common::SystemCommon),
        FixedSize,
        MinSizeUmp(1),
        MinSizeBytes(1)
    )]
    struct ActiveSensing {
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::ux::u4,
    }
}
mod reset {
    use crate::{
        detail::common_properties,
        system_common::{self, UMP_MESSAGE_TYPE},
    };
    pub(crate) const STATUS: u8 = 0xFF;
    /// Reset Message
    ///
    /// See the [module docs](crate::system_common) for more info.
    #[midi2_proc::generate_message(
        Via(system_common::SystemCommon),
        FixedSize,
        MinSizeUmp(1),
        MinSizeBytes(1)
    )]
    struct Reset {
        #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
        ump_type: (),
        #[property(system_common::SystemCommonStatus<{STATUS}>)]
        status: (),
        #[property(common_properties::GroupProperty)]
        group: crate::ux::u4,
    }
}

pub use active_sensing::*;
pub use cont::*;
pub use packet::Packet;
pub use reset::*;
pub use song_position_pointer::*;
pub use song_select::*;
pub use start::*;
pub use stop::*;
pub use time_code::*;
pub use timing_clock::*;
pub use tune_request::*;

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::Packets,
    midi2_proc::Grouped,
    midi2_proc::FromBytes,
    midi2_proc::FromUmp,
    midi2_proc::TryFromBytes,
    midi2_proc::TryFromUmp,
    midi2_proc::RebufferFrom,
    midi2_proc::RebufferFromArray,
    midi2_proc::TryRebufferFrom,
    Clone,
    Copy,
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
    type Error = crate::error::InvalidData;
    fn try_from(buffer: &'a [U]) -> Result<Self, Self::Error> {
        if buffer.is_empty() {
            return Err(crate::error::InvalidData("Slice is too short"));
        };

        Ok(match status(buffer) {
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
            _ => Err(crate::error::InvalidData(
                "Unknown midi1 channel voice status",
            ))?,
        })
    }
}

struct SystemCommonStatus<const STATUS: u8>;

impl<const STATUS: u8, B: crate::buffer::Buffer> crate::detail::property::Property<B>
    for SystemCommonStatus<STATUS>
{
    type Type = ();
}

impl<'a, const STATUS: u8, B: crate::buffer::Buffer> crate::detail::property::ReadProperty<'a, B>
    for SystemCommonStatus<STATUS>
{
    fn read(_buffer: &'a B) -> Self::Type {}
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        if status(buffer.buffer()) != STATUS {
            Err(crate::error::InvalidData("Incorrect status field"))
        } else {
            Ok(())
        }
    }
}

impl<const STATUS: u8, B: crate::buffer::Buffer + crate::buffer::BufferMut>
    crate::detail::property::WriteProperty<B> for SystemCommonStatus<STATUS>
{
    fn write(buffer: &mut B, _: Self::Type) {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U32 => {
                use crate::buffer::SpecialiseU32;
                use crate::detail::BitOps;
                buffer.buffer_mut().specialise_u32_mut()[0].set_octet(1, STATUS);
            }
            crate::buffer::UNIT_ID_U8 => {
                use crate::buffer::SpecialiseU8;
                buffer.buffer_mut().specialise_u8_mut()[0] = STATUS;
            }
            _ => unreachable!(),
        }
    }
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

fn status<U: crate::buffer::Unit>(buffer: &[U]) -> u8 {
    match <U as crate::buffer::UnitPrivate>::UNIT_ID {
        crate::buffer::UNIT_ID_U32 => {
            use crate::detail::BitOps;
            <U as crate::buffer::UnitPrivate>::specialise_buffer_u32(buffer)[0].octet(1)
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
    fn timing_clock_bytes_data() {
        use crate::Data;
        assert_eq!(
            TimingClock::try_from(&[0xF8_u8][..]).unwrap().data(),
            &[0xF8_u8][..]
        );
    }

    #[test]
    fn start_bytes_data() {
        use crate::Data;
        assert_eq!(
            Start::try_from(&[0xFA_u8][..]).unwrap().data(),
            &[0xFA_u8][..]
        );
    }

    #[test]
    fn continue_bytes_data() {
        use crate::Data;
        assert_eq!(
            Continue::try_from(&[0xFB_u8][..]).unwrap().data(),
            &[0xFB_u8][..]
        );
    }

    #[test]
    fn stop_bytes_data() {
        use crate::Data;
        assert_eq!(
            Stop::try_from(&[0xFC_u8][..]).unwrap().data(),
            &[0xFC_u8][..]
        );
    }

    #[test]
    fn active_sensing_bytes_data() {
        use crate::Data;
        assert_eq!(
            ActiveSensing::try_from(&[0xFE_u8][..]).unwrap().data(),
            &[0xFE_u8][..]
        );
    }

    #[test]
    fn reset_bytes_data() {
        use crate::Data;
        assert_eq!(
            Reset::try_from(&[0xFF_u8][..]).unwrap().data(),
            &[0xFF_u8][..]
        );
    }

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

    #[test]
    fn packets() {
        use crate::Packets;

        let message = SystemCommon::try_from(&[0x15F1_5F00_u32][..]).unwrap();
        let mut packets = message.packets();

        assert_eq!(&*packets.next().unwrap(), &[0x15F1_5F00_u32][..]);
        assert_eq!(packets.next(), None);
    }

    #[test]
    fn rebuffer_from_array() {
        use crate::ArrayRebufferFrom;

        let message = SystemCommon::try_from(&[0x15F1_5F00_u32][..]).unwrap();
        let _ = SystemCommon::<[u32; 1]>::array_rebuffer_from(message);
    }

    #[test]
    fn rebuffer_from_array_bytes() {
        use crate::ArrayRebufferFrom;

        let message = SystemCommon::try_from(&[0xF3_u8, 0x4D][..]).unwrap();
        let _ = SystemCommon::<[u8; 3]>::array_rebuffer_from(message);
    }
}
