// pub mod delta_clock_stamp;
// pub mod no_op;
// pub mod time_stamp;

const UMP_MESSAGE_TYPE: u8 = 0x0;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JitterReduction {
    Timestamp(u16),
    DeltaClockstamp(u16),
    DeltaClockstampTPQ(u16),
}

pub(crate) const ERR_JR_UNEXPECTED_CLOCK: &str = "Unexpected clock message";
pub(crate) const ERR_UNKNOWN_UTILITY_STATUS: &str = "Unknown utility message status";
pub(crate) const ERR_JR_HEADER_SHOULD_NOT_EXCEED_ONE_PACKET: &str =
    "The jitter reduction message header must not exceed one packet";

const STATUS_NOOP: u8 = 0b0000;
const STATUS_CLOCK: u8 = 0b0001;
const STATUS_TIMESTAMP: u8 = 0b0010;
const STATUS_DELTA_CLOCKSTAMP_TPQ: u8 = 0b0011;
const STATUS_DELTA_CLOCKSTAMP: u8 = 0b0100;

pub struct JitterReductionProperty;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for JitterReductionProperty {
    type Type = Option<JitterReduction>;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        use crate::buffer::{SpecialiseU32, UmpPrivate, UnitPrivate, UNIT_ID_U32};
        use crate::util::BitOps;

        if <B::Unit as UnitPrivate>::UNIT_ID != UNIT_ID_U32 {
            return Ok(None);
        }

        let buffer = buffer.specialise_u32();
        let jr_slice = buffer.jitter_reduction();
        if jr_slice.is_empty() {
            return Ok(None);
        }

        if buffer.len() > 1 && buffer[1].nibble(0) == crate::numeric_types::u4::new(0) {
            return Err(Error::InvalidData(
                ERR_JR_HEADER_SHOULD_NOT_EXCEED_ONE_PACKET,
            ));
        }

        use crate::error::Error;

        match u8::from(jr_slice[0].nibble(2)) {
            STATUS_CLOCK => return Err(Error::InvalidData(ERR_JR_UNEXPECTED_CLOCK)),
            STATUS_TIMESTAMP => return Ok(Some(JitterReduction::Timestamp(jr_slice[0].word(1)))),
            STATUS_DELTA_CLOCKSTAMP_TPQ => {
                return Ok(Some(JitterReduction::DeltaClockstampTPQ(
                    jr_slice[0].word(1),
                )))
            }
            STATUS_DELTA_CLOCKSTAMP => {
                return Ok(Some(JitterReduction::DeltaClockstamp(jr_slice[0].word(1))))
            }
            STATUS_NOOP => {
                return Ok(None);
            }
            _ => return Err(Error::InvalidData(ERR_UNKNOWN_UTILITY_STATUS)),
        }
    }
    fn write(buffer: &mut B, jr: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        if <B::Unit as UnitPrivate>::UNIT_ID != UNIT_ID_U32 {
            return Ok(());
        }

        use crate::buffer::{SpecialiseU32, UmpPrivateMut, UnitPrivate, UNIT_ID_U32};
        use crate::numeric_types::u4;
        use crate::util::BitOps;
        use JitterReduction::*;

        let mut buffer = buffer.specialise_u32_mut();
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

        Ok(())
    }
    fn default() -> Self::Type {
        None
    }
}

// #[derive(
//     midi2_proc::UmpDebug,
//     derive_more::From,
//     midi2_proc::Data,
//     midi2_proc::Grouped,
//     Clone,
//     PartialEq,
//     Eq,
// )]
// #[non_exhaustive]
// pub enum UtilityMessage<'a> {
//     NoOp(NoOpMessage<'a>),
//     TimeStamp(TimeStampMessage<'a>),
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use pretty_assertions::assert_eq;
//
//     #[test]
//     fn builder() {
//         assert_eq!(
//             UtilityMessage::builder()
//                 .time_stamp()
//                 .time_stamp(u20::new(0x1))
//                 .build(),
//             Ok(UtilityMessage::TimeStamp(
//                 TimeStampMessage::builder()
//                     .time_stamp(u20::new(0x1))
//                     .build()
//                     .unwrap()
//             ))
//         )
//     }
// }
