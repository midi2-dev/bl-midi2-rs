use crate::{util::BitOps, *};

pub mod no_op;
pub mod time_stamp;

pub use no_op::NoOp;
pub use no_op::NoOpBorrowed;
pub use no_op::NoOpBuilder;
pub use no_op::NoOpOwned;
pub use time_stamp::TimeStamp;
pub use time_stamp::TimeStampBorrowed;
pub use time_stamp::TimeStampBuilder;
pub use time_stamp::TimeStampOwned;

pub enum UtilityBorrowed<'a> {
    NoOp(NoOpBorrowed<'a>),
    TimeStamp(TimeStampBorrowed<'a>),
}

pub enum UtilityOwned {
    NoOp(NoOpOwned),
    TimeStamp(TimeStampOwned),
}

const NO_OP_CODE: u8 = 0b0000;
const TIME_STAMP_CODE: u8 = 0b0010;

impl<'a> Data for UtilityBorrowed<'a> {
    fn data(&self) -> &[u32] {
        use UtilityBorrowed::*;
        match self {
            NoOp(m) => m.data(),
            TimeStamp(m) => m.data(),
        }
    }
}

impl Data for UtilityOwned {
    fn data(&self) -> &[u32] {
        use UtilityOwned::*;
        match self {
            NoOp(m) => m.data(),
            TimeStamp(m) => m.data(),
        }
    }
}

impl<'a> Grouped for UtilityBorrowed<'a> {
    fn group(&self) -> u4 {
        use UtilityBorrowed::*;
        match self {
            NoOp(m) => m.group(),
            TimeStamp(m) => m.group(),
        }
    }
}

impl Grouped for UtilityOwned {
    fn group(&self) -> u4 {
        use UtilityOwned::*;
        match self {
            NoOp(m) => m.group(),
            TimeStamp(m) => m.group(),
        }
    }
}

impl<'a> FromData<'a> for UtilityBorrowed<'a> {
    fn validate_data(data: &[u32]) -> Result<()> {
        match u8::from(data[0].nibble(2)) {
            NO_OP_CODE => NoOpBorrowed::validate_data(data),
            TIME_STAMP_CODE => TimeStampBorrowed::validate_data(data),
            _ => Err(Error::InvalidData),
        }
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        use UtilityBorrowed::*;
        match u8::from(data[0].nibble(2)) {
            NO_OP_CODE => NoOp(NoOpBorrowed::from_data_unchecked(data)),
            TIME_STAMP_CODE => TimeStamp(TimeStampBorrowed::from_data_unchecked(data)),
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
