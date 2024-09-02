mod no_op {
    use crate::detail::common_properties;
    use crate::utility;
    pub const STATUS: u8 = 0b0000;
    #[midi2_proc::generate_message(Via(crate::utility::Utility), FixedSize, MinSizeUmp(1))]
    struct NoOp {
        #[property(common_properties::UmpMessageTypeProperty<{utility::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
        status: (),
        #[property(utility::DataProperty)]
        time_data: u16,
    }
}
mod clock {
    use crate::detail::common_properties;
    use crate::utility;
    pub const STATUS: u8 = 0b0001;
    #[midi2_proc::generate_message(Via(crate::utility::Utility), FixedSize, MinSizeUmp(1))]
    struct Clock {
        #[property(common_properties::UmpMessageTypeProperty<{utility::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
        status: (),
        #[property(utility::DataProperty)]
        time_data: u16,
    }
}
mod timestamp {
    use crate::detail::common_properties;
    use crate::utility;
    pub const STATUS: u8 = 0b0010;
    #[midi2_proc::generate_message(Via(crate::utility::Utility), FixedSize, MinSizeUmp(1))]
    struct Timestamp {
        #[property(common_properties::UmpMessageTypeProperty<{utility::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
        status: (),
        #[property(utility::DataProperty)]
        time_data: u16,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::traits::RebufferInto;
        use pretty_assertions::assert_eq;

        #[test]
        fn from_data() {
            assert_eq!(
                Timestamp::try_from(&[0x0020_1234][..]),
                Ok(Timestamp(&[0x0020_1234][..]))
            );
        }

        #[test]
        fn time_data() {
            assert_eq!(
                Timestamp::try_from(&[0x0020_1234][..]).unwrap().time_data(),
                0x1234,
            );
        }

        #[test]
        fn new() {
            let message = Timestamp::<std::vec::Vec<u32>>::new();
            assert_eq!(message, Timestamp(std::vec![0x0020_0000]));
        }

        #[test]
        fn try_new() {
            let message = Timestamp::<[u32; 1]>::try_new();
            assert_eq!(message, Ok(Timestamp([0x0020_0000])));
        }

        #[test]
        fn try_new_fail() {
            let message = Timestamp::<[u32; 0]>::try_new();
            assert_eq!(message, Err(crate::error::BufferOverflow));
        }

        #[test]
        fn new_arr() {
            let message = Timestamp::<[u32; 4]>::new();
            assert_eq!(message, Timestamp([0x0020_0000, 0x0, 0x0, 0x0]));
        }

        #[test]
        fn rebuffer_into() {
            let message = Timestamp::<[u32; 4]>::new();
            let rebuffered: Timestamp<std::vec::Vec<u32>> = message.rebuffer_into();
            assert_eq!(rebuffered, Timestamp(std::vec![0x0020_0000]));
        }
    }
}
mod delta_clockstamp {
    use crate::detail::common_properties;
    use crate::utility;
    pub const STATUS: u8 = 0b0010;
    #[midi2_proc::generate_message(Via(crate::utility::Utility), FixedSize, MinSizeUmp(1))]
    struct DeltaClockstamp {
        #[property(common_properties::UmpMessageTypeProperty<{utility::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
        status: (),
        #[property(utility::DataProperty)]
        time_data: u16,
    }
}
mod delta_clockstamp_tpq {
    use crate::detail::common_properties;
    use crate::utility;
    pub const STATUS: u8 = 0b0011;
    #[midi2_proc::generate_message(Via(crate::utility::Utility), FixedSize, MinSizeUmp(1))]
    struct DeltaClockstampTpq {
        #[property(common_properties::UmpMessageTypeProperty<{utility::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
        status: (),
        #[property(utility::DataProperty)]
        time_data: u16,
    }
}

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x0;

pub use clock::Clock;
pub use delta_clockstamp::DeltaClockstamp;
pub use delta_clockstamp_tpq::DeltaClockstampTpq;
pub use no_op::NoOp;
pub use timestamp::Timestamp;

struct DataProperty;

impl<B: crate::buffer::Ump> crate::detail::property::Property<B> for DataProperty {
    type Type = u16;
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B> for DataProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::detail::BitOps;
        buffer.buffer()[0].word(1)
    }
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for DataProperty
{
    fn write(buffer: &mut B, value: Self::Type) {
        use crate::detail::BitOps;
        buffer.buffer_mut()[0].set_word(1, value);
    }
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::Packets,
    midi2_proc::RebufferFrom,
    midi2_proc::RebufferFromArray,
    midi2_proc::TryRebufferFrom,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[non_exhaustive]
pub enum Utility<B: crate::buffer::Ump> {
    NoOp(no_op::NoOp<B>),
    Clock(clock::Clock<B>),
    Timestamp(timestamp::Timestamp<B>),
    DeltaClockstamp(delta_clockstamp::DeltaClockstamp<B>),
    DeltaClockstampTpq(delta_clockstamp_tpq::DeltaClockstampTpq<B>),
}

impl<'a> core::convert::TryFrom<&'a [u32]> for Utility<&'a [u32]> {
    type Error = crate::error::InvalidData;
    fn try_from(buffer: &'a [u32]) -> Result<Self, Self::Error> {
        if buffer.is_empty() {
            return Err(crate::error::InvalidData("Slice is too short"));
        };
        Ok(match status(buffer) {
            no_op::STATUS => no_op::NoOp::try_from(buffer)?.into(),
            clock::STATUS => clock::Clock::try_from(buffer)?.into(),
            timestamp::STATUS => timestamp::Timestamp::try_from(buffer)?.into(),
            delta_clockstamp::STATUS => delta_clockstamp::DeltaClockstamp::try_from(buffer)?.into(),
            delta_clockstamp_tpq::STATUS => {
                delta_clockstamp_tpq::DeltaClockstampTpq::try_from(buffer)?.into()
            }
            _ => Err(crate::error::InvalidData("Unknown utility message status"))?,
        })
    }
}

fn status<U: crate::buffer::Unit>(buffer: &[U]) -> u8 {
    use crate::detail::BitOps;
    match <U as crate::buffer::UnitPrivate>::UNIT_ID {
        crate::buffer::UNIT_ID_U8 => {
            <U as crate::buffer::UnitPrivate>::specialise_buffer_u8(buffer)[0].nibble(0)
        }
        crate::buffer::UNIT_ID_U32 => {
            <U as crate::buffer::UnitPrivate>::specialise_buffer_u32(buffer)[0].nibble(2)
        }
        _ => unreachable!(),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_data() {
        assert_eq!(
            Utility::try_from(&[0x0010_1234][..]),
            Ok(Utility::Clock(
                clock::Clock::try_from(&[0x0010_1234][..]).unwrap()
            ))
        );
    }

    #[test]
    fn packets() {
        use crate::Packets;

        let message = Utility::try_from(&[0x0010_1234][..]).unwrap();

        let mut packets = message.packets();
        assert_eq!(packets.next(), Some(&[0x0010_1234][..]));
        assert_eq!(packets.next(), None);
    }

    #[test]
    fn rebuffer_from() {
        use crate::RebufferFrom;

        let message = Utility::try_from(&[0x0010_1234][..]).unwrap();
        let _ = Utility::<[u32; 1]>::rebuffer_from(message);
    }

    #[test]
    fn delta_clock_stamp_try_from() {
        DeltaClockstamp::try_from(&[0x0020_0000][..]).unwrap();
    }
}
