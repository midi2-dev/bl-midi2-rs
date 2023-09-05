use crate::{util::BitOps, *};

pub mod no_op;
pub mod time_stamp;

pub use no_op::NoOpBuilder;
pub use no_op::NoOpMessage;
pub use time_stamp::TimeStampBuilder;
pub use time_stamp::TimeStampMessage;

pub enum UtilityMessage<'a> {
    NoOp(NoOpMessage<'a>),
    TimeStamp(TimeStampMessage<'a>),
}

const NO_OP_CODE: u8 = 0b0000;
const TIME_STAMP_CODE: u8 = 0b0010;

impl<'a> Message<'a, Ump> for UtilityMessage<'a> {
    fn data(&self) -> &'a [u32] {
        use UtilityMessage::*;
        match self {
            NoOp(m) => m.data(),
            TimeStamp(m) => m.data(),
        }
    }
    fn validate_data(data: &[u32]) -> Result<()> {
        match u8::from(data[0].nibble(2)) {
            NO_OP_CODE => NoOpMessage::validate_data(data),
            TIME_STAMP_CODE => TimeStampMessage::validate_data(data),
            _ => Err(Error::InvalidData),
        }
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        use UtilityMessage::*;
        match u8::from(data[0].nibble(2)) {
            NO_OP_CODE => NoOp(NoOpMessage::from_data_unchecked(data)),
            TIME_STAMP_CODE => TimeStamp(TimeStampMessage::from_data_unchecked(data)),
            _ => panic!(),
        }
    }
}

pub fn validate_packet(p: &[u32], op_code: u4) -> Result<()> {
    if p.is_empty() {
        Err(Error::BufferOverflow)
    } else if p[0].nibble(0) != u4::new(0x0) || p[0].nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
