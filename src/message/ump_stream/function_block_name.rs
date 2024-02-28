use super::ump_stream_group::{
    PayloadIterator as UmpStreamGroupPayloadIterator, UmpStreamGroup, UmpStreamGroupBorrowed,
    UmpStreamGroupBorrowedBuilder,
};
#[cfg(feature = "std")]
use super::ump_stream_group::{UmpStreamGroupBuilder, UmpStreamGroupOwned};
use crate::{
    numeric_types::*,
    traits::{Data, FromData},
    util::BitOps,
    Error, Result,
};
#[cfg(feature = "std")]
use crate::{IntoOwned, Level2Message};

const STATUS: u16 = 0x12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionBlockNameBorrowed<'a>(UmpStreamGroupBorrowed<'a>);

#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionBlockNameOwned(UmpStreamGroupOwned);

#[derive(derive_more::From, Debug, Clone, PartialEq, Eq)]
pub enum FunctionBlockNameMessage<'a> {
    Borrowed(FunctionBlockNameBorrowed<'a>),
    #[cfg(feature = "std")]
    Owned(FunctionBlockNameOwned),
}

#[cfg(feature = "std")]
pub struct FunctionBlockNameBuilder<M: core::convert::From<FunctionBlockNameOwned>>(
    UmpStreamGroupBuilder<UmpStreamGroupOwned>,
    core::marker::PhantomData<M>,
);

pub struct FunctionBlockNameBorrowedBuilder<'a>(UmpStreamGroupBorrowedBuilder<'a>);

type EnumeratedPayloadIterator<'a> = core::iter::Enumerate<UmpStreamGroupPayloadIterator<'a>>;
type NameFilter = fn((usize, u8)) -> Option<u8>;
pub struct NameBytesIterator<'a>(core::iter::FilterMap<EnumeratedPayloadIterator<'a>, NameFilter>);

pub trait FunctionBlockName: Data {
    #[cfg(feature = "std")]
    fn name(&self) -> core::result::Result<std::string::String, std::string::FromUtf8Error> {
        std::string::String::from_utf8(self.name_bytes().collect())
    }

    fn name_bytes(&self) -> NameBytesIterator {
        let group = UmpStreamGroupBorrowed::from_data_unchecked(self.data());
        NameBytesIterator(group.payload().enumerate().filter_map(filter_name_bytes))
    }

    fn function_block(&self) -> u8 {
        function_block_from_packet(self.data())
    }
}

impl<'a> core::iter::Iterator for NameBytesIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(feature = "std")]
impl<'a> FunctionBlockNameMessage<'a> {
    pub fn builder() -> FunctionBlockNameBuilder<Self> {
        FunctionBlockNameBuilder::new()
    }
}

impl<'a> FunctionBlockNameBorrowed<'a> {
    pub fn builder(buffer: &'a mut [u32]) -> FunctionBlockNameBorrowedBuilder<'a> {
        FunctionBlockNameBorrowedBuilder::new(buffer)
    }
}

impl<'a> FunctionBlockName for FunctionBlockNameBorrowed<'a> {}
impl<'a> FunctionBlockName for FunctionBlockNameMessage<'a> {}
#[cfg(feature = "std")]
impl FunctionBlockName for FunctionBlockNameOwned {}

impl<'a> Data for FunctionBlockNameBorrowed<'a> {
    fn data(&self) -> &[u32] {
        self.0.data()
    }
}

#[cfg(feature = "std")]
impl Data for FunctionBlockNameOwned {
    fn data(&self) -> &[u32] {
        self.0.data()
    }
}

impl<'a> FromData<'a> for FunctionBlockNameBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Self(UmpStreamGroupBorrowed::from_data_unchecked(buffer))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        UmpStreamGroupBorrowed::validate_data(buffer)?;
        // correct status
        if super::status_from_buffer(buffer) != u10::new(STATUS) {
            return Err(Error::InvalidData);
        }
        // consistent function block numbers
        let function_block = function_block_from_packet(buffer);
        if !buffer
            .chunks_exact(4)
            .all(|packet| function_block_from_packet(packet) == function_block)
        {
            return Err(Error::InvalidData);
        }
        Ok(())
    }
}

impl<'a> FromData<'a> for FunctionBlockNameMessage<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Self::Borrowed(FunctionBlockNameBorrowed::from_data_unchecked(buffer))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        FunctionBlockNameBorrowed::validate_data(buffer)
    }
}

impl<'a> Data for FunctionBlockNameMessage<'a> {
    fn data(&self) -> &[u32] {
        use FunctionBlockNameMessage::*;
        match self {
            #[cfg(feature = "std")]
            Owned(m) => m.data(),
            Borrowed(m) => m.data(),
        }
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<FunctionBlockNameOwned>> core::default::Default
    for FunctionBlockNameBuilder<M>
{
    fn default() -> Self {
        Self::new()
    }
}

struct InterleaveFunctionBlockIterator<'a> {
    iter: core::str::Bytes<'a>,
    function_block: u8,
    packet_index: usize,
}

impl<'a> core::iter::Iterator for InterleaveFunctionBlockIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.packet_index == 0 {
            Some(self.function_block)
        } else {
            self.iter.next()
        };

        self.packet_index += 1;
        self.packet_index %= 14;

        ret
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<FunctionBlockNameOwned>> FunctionBlockNameBuilder<M> {
    pub fn new() -> Self {
        Self(
            {
                let mut builder = UmpStreamGroupBuilder::new();
                builder.status(u10::new(STATUS));
                builder
            },
            Default::default(),
        )
    }
    pub fn build(&self) -> Result<M> {
        match self.0.build() {
            Ok(m) => Ok(FunctionBlockNameOwned(m).into()),
            Err(e) => Err(e),
        }
    }
    pub fn name(&mut self, name_str: &str) -> &mut Self {
        let function_block = function_block_from_packet(&self.0.buffer);
        self.0.payload(InterleaveFunctionBlockIterator {
            iter: name_str.bytes(),
            function_block,
            packet_index: 0,
        });
        self
    }

    pub fn function_block(&mut self, v: u8) -> &mut Self {
        for chunk in self.0.buffer.chunks_exact_mut(4) {
            chunk[0] &= !0x0000_FF00;
            chunk[0] |= u32::from(v) << 8;
        }
        self
    }
}

impl<'a> FunctionBlockNameBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Self(UmpStreamGroupBorrowedBuilder::new(buffer).status(u10::new(STATUS)))
    }
    pub fn build(self) -> Result<FunctionBlockNameBorrowed<'a>> {
        match self.0.build() {
            Ok(m) => Ok(FunctionBlockNameBorrowed(m)),
            Err(e) => Err(e),
        }
    }
    pub fn name(mut self, name_str: &str) -> Self {
        let function_block = function_block_from_packet(self.0.buffer);
        self.0 = self.0.payload(InterleaveFunctionBlockIterator {
            iter: name_str.bytes(),
            function_block,
            packet_index: 0,
        });
        self
    }

    pub fn function_block(self, v: u8) -> Self {
        for chunk in self.0.buffer.chunks_exact_mut(4) {
            chunk[0] &= !0x0000_FF00;
            chunk[0] |= u32::from(v) << 8;
        }
        self
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for FunctionBlockNameBorrowed<'a> {
    type Owned = FunctionBlockNameOwned;
    fn into_owned(self) -> Self::Owned {
        FunctionBlockNameOwned(self.0.into_owned())
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for FunctionBlockNameMessage<'a> {
    type Owned = FunctionBlockNameOwned;
    fn into_owned(self) -> FunctionBlockNameOwned {
        use FunctionBlockNameMessage::*;
        match self {
            Owned(m) => m,
            Borrowed(m) => m.into_owned(),
        }
    }
}

#[cfg(feature = "std")]
impl Level2Message for FunctionBlockNameOwned {}

fn filter_name_bytes((i, v): (usize, u8)) -> Option<u8> {
    if i % 14 != 0 && v != 0x0 {
        Some(v)
    } else {
        None
    }
}

fn function_block_from_packet(packet: &[u32]) -> u8 {
    packet[0].octet(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buffer::Ump,
        test_support::{debug, random_buffer::RandomBuffer},
    };
    use pretty_assertions::assert_eq;

    #[test]
    #[cfg(feature = "std")]
    fn builder() {
        assert_eq!(
            debug::Data(
                FunctionBlockNameMessage::builder()
                    .name("SynthWave🌊²")
                    .function_block(0x9)
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0xF412_0953,
                0x796E_7468,
                0x5761_7665,
                0xF09F_8C8A,
                0xFC12_09C2,
                0xB200_0000,
                0x0000_0000,
                0x0000_0000,
            ])
        );
    }

    #[test]
    fn borrowed_builder() {
        assert_eq!(
            debug::Data(
                FunctionBlockNameBorrowed::builder(&mut Ump::random_buffer::<8>())
                    .name("SynthWave🌊²")
                    .function_block(0x9)
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0xF412_0953,
                0x796E_7468,
                0x5761_7665,
                0xF09F_8C8A,
                0xFC12_09C2,
                0xB200_0000,
                0x0000_0000,
                0x0000_0000,
            ])
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn name() {
        assert_eq!(
            FunctionBlockNameMessage::from_data(&[
                0xF412_0953,
                0x796E_7468,
                0x5761_7665,
                0xF09F_8C8A,
                0xFC12_09C2,
                0xB200_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .name(),
            Ok(std::string::String::from("SynthWave🌊²")),
        );
    }

    #[test]
    fn function_block() {
        assert_eq!(
            FunctionBlockNameMessage::from_data(&[
                0xF412_0953,
                0x796E_7468,
                0x5761_7665,
                0xF09F_8C8A,
                0xFC12_09C2,
                0xB200_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .function_block(),
            0x9,
        );
    }
}
