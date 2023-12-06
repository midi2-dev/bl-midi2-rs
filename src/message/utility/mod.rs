use crate::{util::BitOps, *};

pub mod no_op;
pub mod time_stamp;

use no_op::NoOpBorrowed;
use no_op::NoOpBuilder;
use no_op::NoOpMessage;
use no_op::NoOpOwned;
use time_stamp::TimeStampBorrowed;
use time_stamp::TimeStampBuilder;
use time_stamp::TimeStampMessage;
use time_stamp::TimeStampOwned;

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UtilityMessage<'a> {
    NoOp(NoOpMessage<'a>),
    TimeStamp(TimeStampMessage<'a>),
}

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UtilityBorrowed<'a> {
    NoOp(NoOpBorrowed<'a>),
    TimeStamp(TimeStampBorrowed<'a>),
}

#[derive(derive_more::From, midi2_attr::Data, midi2_attr::Grouped, Clone, Debug, PartialEq, Eq)]
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
    pub fn builder() -> UtilityBuilder<Self> {
        UtilityBuilder::new()
    }
}

impl<'a> UtilityMessage<'a> {
    pub fn builder() -> UtilityBuilder<Self> {
        UtilityBuilder::new()
    }
}

const NO_OP_CODE: u8 = 0b0000;
const TIME_STAMP_CODE: u8 = 0b0010;

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

impl<'a> FromData<'a> for UtilityMessage<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        UtilityBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        UtilityBorrowed::from_data_unchecked(buffer).into()
    }
}

impl<'a> core::convert::From<UtilityBorrowed<'a>> for UtilityMessage<'a> {
    fn from(value: UtilityBorrowed<'a>) -> Self {
        match value {
            UtilityBorrowed::NoOp(m) => UtilityMessage::NoOp(m.into()),
            UtilityBorrowed::TimeStamp(m) => UtilityMessage::TimeStamp(m.into()),
        }
    }
}

impl<'a> core::convert::From<UtilityOwned> for UtilityMessage<'a> {
    fn from(value: UtilityOwned) -> Self {
        match value {
            UtilityOwned::NoOp(m) => UtilityMessage::NoOp(m.into()),
            UtilityOwned::TimeStamp(m) => UtilityMessage::TimeStamp(m.into()),
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

impl<'a, M> core::convert::From<M> for UtilityMessage<'a>
where
    M: Level2Message,
    UtilityOwned: core::convert::From<M>,
{
    fn from(value: M) -> Self {
        <UtilityOwned as core::convert::From<M>>::from(value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            UtilityMessage::builder()
                .time_stamp()
                .time_stamp(u20::new(0x1))
                .build(),
            Ok(UtilityMessage::TimeStamp(
                TimeStampMessage::builder()
                    .time_stamp(u20::new(0x1))
                    .build()
                    .unwrap()
            ))
        )
    }
}

impl<'a> IntoOwned for UtilityBorrowed<'a> {
    type Owned = UtilityOwned;
    fn into_owned(self) -> Self::Owned {
        use UtilityBorrowed as B;
        use UtilityOwned as O;
        match self {
            B::NoOp(m) => O::NoOp(m.into_owned()),
            B::TimeStamp(m) => O::TimeStamp(m.into_owned()),
        }
    }
}

impl<'a> IntoOwned for UtilityMessage<'a> {
    type Owned = UtilityOwned;
    fn into_owned(self) -> Self::Owned {
        use UtilityMessage as M;
        use UtilityOwned as O;
        match self {
            M::NoOp(m) => O::NoOp(m.into_owned()),
            M::TimeStamp(m) => O::TimeStamp(m.into_owned()),
        }
    }
}
