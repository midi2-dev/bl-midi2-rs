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

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UtilityBorrowed<'a> {
    NoOp(NoOpBorrowed<'a>),
    TimeStamp(TimeStampBorrowed<'a>),
}

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UtilityOwned {
    NoOp(NoOpOwned),
    TimeStamp(TimeStampOwned),
}

#[derive(Default)]
pub struct UtilityBuilder<M>(core::marker::PhantomData<M>)
where
    M: core::convert::From<TimeStampOwned> + core::convert::From<NoOpOwned>;

impl<M> UtilityBuilder<M>
where
    M: core::convert::From<TimeStampOwned> + core::convert::From<NoOpOwned>,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn no_op(self) -> NoOpBuilder<M> {
        NoOpBuilder::new()
    }
    pub fn time_stamp(self) -> TimeStampBuilder<M> {
        TimeStampBuilder::new()
    }
}

impl UtilityOwned {
    pub fn builder() -> UtilityBuilder<UtilityOwned> {
        UtilityBuilder::new()
    }
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
    type Target = Self;
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            UtilityOwned::builder()
                .time_stamp()
                .time_stamp(u20::new(0x1))
                .build(),
            Ok(UtilityOwned::TimeStamp(
                TimeStampOwned::builder()
                    .time_stamp(u20::new(0x1))
                    .build()
                    .unwrap()
            ))
        )
    }
}

impl<'a> ToOwned for UtilityBorrowed<'a> {
    type Owned = UtilityOwned;
    fn to_owned(self) -> Self::Owned {
        use UtilityBorrowed as B;
        use UtilityOwned as O;
        match self {
            B::NoOp(m) => O::NoOp(m.to_owned()),
            B::TimeStamp(m) => O::TimeStamp(m.to_owned()),
        }
    }
}
