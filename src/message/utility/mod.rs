pub mod no_op {
    use crate::message::utility;
    pub const STATUS: u8 = 0b0000;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(0))]
    struct NoOp {
        #[property(utility::TypeProperty)]
        ump_type: (),
        #[property(utility::StatusProperty<{STATUS}>)]
        status: (),
        #[property(utility::DataProperty)]
        time_data: u16,
    }
}
pub mod clock {
    use crate::message::utility;
    pub const STATUS: u8 = 0b0001;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(0))]
    struct Clock {
        #[property(utility::TypeProperty)]
        ump_type: (),
        #[property(utility::StatusProperty<{STATUS}>)]
        status: (),
        #[property(utility::DataProperty)]
        time_data: u16,
    }
}
pub mod timestamp {
    use crate::message::utility;
    pub const STATUS: u8 = 0b0010;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(0))]
    struct Timestamp {
        #[property(utility::TypeProperty)]
        ump_type: (),
        #[property(utility::StatusProperty<{STATUS}>)]
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
            let message = Timestamp::<[u32; 5]>::new_arr();
            assert_eq!(message, Timestamp([0x0020_0000, 0x0, 0x0, 0x0, 0x0]));
        }

        #[test]
        fn rebuffer_into() {
            let message = Timestamp::<[u32; 5]>::new_arr();
            let rebuffered: Timestamp<std::vec::Vec<u32>> = message.rebuffer_into();
            assert_eq!(rebuffered, Timestamp(std::vec![0x0020_0000]));
        }
    }
}
pub mod delta_clockstamp {
    use crate::message::utility;
    pub const STATUS: u8 = 0b0100;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(0))]
    struct DeltaClockstamp {
        #[property(utility::TypeProperty)]
        ump_type: (),
        #[property(utility::StatusProperty<{STATUS}>)]
        status: (),
    }
}
pub mod delta_clockstamp_tpq {
    use crate::message::utility;
    pub const STATUS: u8 = 0b0011;
    #[midi2_proc::generate_message(FixedSize, MinSizeUmp(0))]
    struct DeltaClockstampTPQ {
        #[property(utility::TypeProperty)]
        ump_type: (),
        #[property(utility::StatusProperty<{STATUS}>)]
        status: (),
        #[property(utility::DataProperty)]
        time_data: u16,
    }
}

const UMP_MESSAGE_TYPE: u8 = 0x0;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JitterReduction {
    Timestamp(u16),
    DeltaClockstamp(u16),
    DeltaClockstampTPQ(u16),
}

pub(crate) const ERR_JR_UNEXPECTED_CLOCK: &str = "Unexpected clock message";
pub(crate) const ERR_UNKNOWN_UTILITY_STATUS: &str = "Unknown utility message status";

const STATUS_NOOP: u8 = 0b0000;
const STATUS_CLOCK: u8 = 0b0001;
const STATUS_TIMESTAMP: u8 = 0b0010;
const STATUS_DELTA_CLOCKSTAMP_TPQ: u8 = 0b0011;
const STATUS_DELTA_CLOCKSTAMP: u8 = 0b0100;

pub struct JitterReductionProperty;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for JitterReductionProperty {
    type Type = Option<JitterReduction>;
}

impl<'a, B: crate::buffer::Buffer> crate::util::property::ReadProperty<'a, B>
    for JitterReductionProperty
{
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::{SpecialiseU32, UmpPrivate, UnitPrivate, UNIT_ID_U32};
        use crate::util::BitOps;

        if <B::Unit as UnitPrivate>::UNIT_ID != UNIT_ID_U32 {
            return None;
        }

        let buffer = buffer.specialise_u32();
        let jr_slice = buffer.jitter_reduction();
        if jr_slice.is_empty() {
            return None;
        }

        match u8::from(jr_slice[0].nibble(2)) {
            STATUS_TIMESTAMP => Some(JitterReduction::Timestamp(jr_slice[0].word(1))),
            STATUS_DELTA_CLOCKSTAMP_TPQ => {
                Some(JitterReduction::DeltaClockstampTPQ(jr_slice[0].word(1)))
            }
            STATUS_DELTA_CLOCKSTAMP => Some(JitterReduction::DeltaClockstamp(jr_slice[0].word(1))),
            STATUS_NOOP => None,
            _ => panic!("Packet in a bad state"),
        }
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::{SpecialiseU32, UmpPrivate, UnitPrivate, UNIT_ID_U32};
        use crate::util::BitOps;

        if <B::Unit as UnitPrivate>::UNIT_ID != UNIT_ID_U32 {
            return Ok(());
        }

        let buffer = buffer.specialise_u32();
        let jr_slice = buffer.jitter_reduction();
        if jr_slice.is_empty() {
            return Ok(());
        }

        // is this needed?
        // if buffer.len() > 1 && buffer[1].nibble(0) == crate::numeric_types::u4::new(0) {
        //     return Err(Error::InvalidData(
        //         ERR_JR_HEADER_SHOULD_NOT_EXCEED_ONE_PACKET,
        //     ));
        // }

        use crate::error::Error;

        match u8::from(jr_slice[0].nibble(2)) {
            STATUS_CLOCK => return Err(Error::InvalidData(ERR_JR_UNEXPECTED_CLOCK)),
            STATUS_TIMESTAMP => return Ok(()),
            STATUS_DELTA_CLOCKSTAMP_TPQ => return Ok(()),
            STATUS_DELTA_CLOCKSTAMP => return Ok(()),
            STATUS_NOOP => {
                return Ok(());
            }
            _ => return Err(Error::InvalidData(ERR_UNKNOWN_UTILITY_STATUS)),
        }
    }
}

impl<B: crate::buffer::Buffer + crate::buffer::BufferMut> crate::util::property::WriteProperty<B>
    for JitterReductionProperty
{
    fn write(buffer: &mut B, jr: Self::Type) {
        use crate::buffer::{SpecialiseU32, UmpPrivateMut, UnitPrivate, UNIT_ID_U32};
        use crate::numeric_types::u4;
        use crate::util::BitOps;
        use JitterReduction::*;

        if <B::Unit as UnitPrivate>::UNIT_ID != UNIT_ID_U32 {
            return;
        }

        let buffer = buffer.specialise_u32_mut();
        let jr_slice = buffer.jitter_reduction_mut();
        debug_assert!(!jr_slice.is_empty());
        match jr {
            Some(Timestamp(data)) => {
                jr_slice[0].set_nibble(2, u4::new(STATUS_TIMESTAMP));
                jr_slice[0].set_word(1, data);
            }
            Some(DeltaClockstampTPQ(data)) => {
                jr_slice[0].set_nibble(2, u4::new(STATUS_DELTA_CLOCKSTAMP_TPQ));
                jr_slice[0].set_word(1, data);
            }
            Some(DeltaClockstamp(data)) => {
                jr_slice[0].set_nibble(2, u4::new(STATUS_DELTA_CLOCKSTAMP));
                jr_slice[0].set_word(1, data);
            }
            None => {
                jr_slice[0] = 0x0;
            }
        }
    }
    fn validate(_: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        None
    }
}

struct TypeProperty;

impl<B: crate::buffer::Ump> crate::util::property::Property<B> for TypeProperty {
    type Type = ();
}

impl<'a, B: crate::buffer::Ump> crate::util::property::ReadProperty<'a, B> for TypeProperty {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::util::BitOps;
        if buffer.buffer()[0].nibble(0) != crate::u4::new(0x0) {
            Err(crate::Error::InvalidData("Incorrect ump message type"))
        } else {
            Ok(())
        }
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::util::property::WriteProperty<B>
    for TypeProperty
{
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::util::BitOps;
        buffer.buffer_mut()[0].set_nibble(0, crate::u4::new(0x0));
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}
struct StatusProperty<const STATUS: u8>;

impl<const STATUS: u8, B: crate::buffer::Ump> crate::util::property::Property<B>
    for StatusProperty<STATUS>
{
    type Type = ();
}

impl<'a, const STATUS: u8, B: crate::buffer::Ump> crate::util::property::ReadProperty<'a, B>
    for StatusProperty<STATUS>
{
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::util::BitOps;
        if u8::from(buffer.buffer()[0].nibble(2)) == STATUS {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message status"))
        }
    }
}

impl<const STATUS: u8, B: crate::buffer::Ump + crate::buffer::BufferMut>
    crate::util::property::WriteProperty<B> for StatusProperty<STATUS>
{
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::util::BitOps;
        buffer.buffer_mut()[0].set_nibble(2, crate::u4::new(STATUS));
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

struct DataProperty;

impl<B: crate::buffer::Ump> crate::util::property::Property<B> for DataProperty {
    type Type = u16;
}

impl<'a, B: crate::buffer::Ump> crate::util::property::ReadProperty<'a, B> for DataProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::util::BitOps;
        buffer.buffer()[0].word(1)
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::util::property::WriteProperty<B>
    for DataProperty
{
    fn write(buffer: &mut B, value: Self::Type) {
        use crate::util::BitOps;
        buffer.buffer_mut()[0].set_word(1, value);
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::RebufferFrom,
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
    DeltaClockstampTpq(delta_clockstamp_tpq::DeltaClockstampTPQ<B>),
}

impl<'a> core::convert::TryFrom<&'a [u32]> for Utility<&'a [u32]> {
    type Error = crate::error::Error;
    fn try_from(buffer: &'a [u32]) -> Result<Self, Self::Error> {
        if buffer.len() < 1 {
            return Err(crate::error::Error::InvalidData("Slice is too short"));
        };
        Ok(match status(buffer) {
            no_op::STATUS => no_op::NoOp::try_from(buffer)?.into(),
            clock::STATUS => clock::Clock::try_from(buffer)?.into(),
            timestamp::STATUS => timestamp::Timestamp::try_from(buffer)?.into(),
            delta_clockstamp::STATUS => delta_clockstamp::DeltaClockstamp::try_from(buffer)?.into(),
            delta_clockstamp_tpq::STATUS => {
                delta_clockstamp_tpq::DeltaClockstampTPQ::try_from(buffer)?.into()
            }
            _ => Err(crate::error::Error::InvalidData(
                "Unknown utility message status",
            ))?,
        })
    }
}

fn status<U: crate::buffer::Unit>(buffer: &[U]) -> u8 {
    use crate::util::BitOps;
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
}
