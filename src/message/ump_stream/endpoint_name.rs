use super::ump_stream_group::{
    PayloadIterator as UmpStreamGroupPayloadIterator, UmpStreamGroup, UmpStreamGroupBorrowed,
    UmpStreamGroupBorrowedBuilder,
};
#[cfg(feature = "std")]
use super::ump_stream_group::{UmpStreamGroupBuilder, UmpStreamGroupOwned};
use crate::{
    numeric_types::*,
    traits::{Data, FromData},
    Error, Result,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointNameBorrowed<'a>(UmpStreamGroupBorrowed<'a>);

#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointNameOwned(UmpStreamGroupOwned);

#[derive(derive_more::From, Debug, Clone, PartialEq, Eq)]
pub enum EndpointNameMessage<'a> {
    Borrowed(EndpointNameBorrowed<'a>),
    #[cfg(feature = "std")]
    Owned(EndpointNameOwned),
}

#[cfg(feature = "std")]
pub struct EndpointNameBuilder<M: core::convert::From<EndpointNameOwned>>(
    UmpStreamGroupBuilder<UmpStreamGroupOwned>,
    core::marker::PhantomData<M>,
);

pub struct EndpointNameBorrowedBuilder<'a>(UmpStreamGroupBorrowedBuilder<'a>);

pub struct NameBytesIterator<'a>(
    core::iter::Filter<UmpStreamGroupPayloadIterator<'a>, fn(&u8) -> bool>,
);

pub trait EndpointName: Data {
    #[cfg(feature = "std")]
    fn name(&self) -> std::string::String {
        let group = UmpStreamGroupBorrowed::from_data_unchecked(self.data());
        std::string::String::from_utf8(group.payload().filter(|v| *v != 0x0).collect()).unwrap()
    }

    fn name_utf8_bytes(&self) -> NameBytesIterator {
        let group = UmpStreamGroupBorrowed::from_data_unchecked(self.data());
        NameBytesIterator(group.payload().filter(|v| *v != 0x0))
    }
}

impl<'a> core::iter::Iterator for NameBytesIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(feature = "std")]
impl<'a> EndpointNameMessage<'a> {
    pub fn builder() -> EndpointNameBuilder<Self> {
        EndpointNameBuilder::new()
    }
}

impl<'a> EndpointNameBorrowed<'a> {
    pub fn builder(buffer: &'a mut [u32]) -> EndpointNameBorrowedBuilder<'a> {
        EndpointNameBorrowedBuilder::new(buffer)
    }
}

impl<'a> EndpointName for EndpointNameBorrowed<'a> {}
impl<'a> EndpointName for EndpointNameMessage<'a> {}
#[cfg(feature = "std")]
impl EndpointName for EndpointNameOwned {}

impl<'a> Data for EndpointNameBorrowed<'a> {
    fn data(&self) -> &[u32] {
        self.0.data()
    }
}

#[cfg(feature = "std")]
impl Data for EndpointNameOwned {
    fn data(&self) -> &[u32] {
        self.0.data()
    }
}

impl<'a> FromData<'a> for EndpointNameBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Self(UmpStreamGroupBorrowed::from_data_unchecked(buffer))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        UmpStreamGroupBorrowed::validate_data(buffer)?;
        if super::status_from_buffer(buffer) != u10::new(0x3) {
            return Err(Error::InvalidData);
        }
        Ok(())
    }
}

impl<'a> FromData<'a> for EndpointNameMessage<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Self::Borrowed(EndpointNameBorrowed::from_data_unchecked(buffer))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        EndpointNameBorrowed::validate_data(buffer)
    }
}

impl<'a> Data for EndpointNameMessage<'a> {
    fn data(&self) -> &[u32] {
        use EndpointNameMessage::*;
        match self {
            #[cfg(feature = "std")]
            Owned(m) => m.data(),
            Borrowed(m) => m.data(),
        }
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<EndpointNameOwned>> core::default::Default for EndpointNameBuilder<M> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<EndpointNameOwned>> EndpointNameBuilder<M> {
    pub fn new() -> Self {
        Self(
            UmpStreamGroupBuilder::new().status(u10::new(0x3)),
            Default::default(),
        )
    }
    pub fn build(self) -> Result<M> {
        match self.0.build() {
            Ok(m) => Ok(EndpointNameOwned(m).into()),
            Err(e) => Err(e),
        }
    }
    pub fn name(mut self, name_str: &str) -> Self {
        self.0 = self.0.payload(name_str.bytes());
        self
    }
}

impl<'a> EndpointNameBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Self(UmpStreamGroupBorrowedBuilder::new(buffer).status(u10::new(0x3)))
    }
    pub fn build(self) -> Result<EndpointNameBorrowed<'a>> {
        match self.0.build() {
            Ok(m) => Ok(EndpointNameBorrowed(m)),
            Err(e) => Err(e),
        }
    }
    pub fn name(mut self, name_str: &str) -> Self {
        self.0 = self.0.payload(name_str.bytes());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        util::{debug, RandomBuffer},
        Ump,
    };
    use pretty_assertions::assert_eq;

    #[cfg(feature = "std")]
    #[test]
    fn test() {
        assert_eq!(
            std::string::String::from_utf8(std::vec![
                0x47, 0x69, 0x6D, 0x6D, 0x65, 0x20, 0x73, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x69, 0x67,
                0x6E, 0x61, 0x6C, 0x20, 0xF0, 0x9F, 0x94, 0x8A, 0x20, 0xF0, 0x9F, 0x99, 0x8C,
            ]),
            Ok("Gimme some signal 🔊 🙌".into())
        );
        assert_eq!(
            std::string::String::from_utf8(std::vec![
                0x47, 0x69, 0x6D, 0x6D, 0x65, 0x20, 0x73, 0x6F, 0x6D, 0x65, 0x20, 0x6D, 0x6F, 0x72,
                0x65, 0x20, 0x73, 0x69, 0x67, 0x6E, 0x61, 0x6C, 0x20, 0xF0, 0x9F, 0x94, 0x8A, 0x20,
                0xF0, 0x9F, 0x99, 0x8C
            ]),
            Ok("Gimme some more signal 🔊 🙌".into())
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn builder() {
        assert_eq!(
            debug::Data(
                EndpointNameMessage::builder()
                    .name("Gimme some signal 🔊 🙌")
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0xF403_4769,
                0x6D6D_6520,
                0x736F_6D65,
                0x2073_6967,
                0xFC03_6E61,
                0x6C20_F09F,
                0x948A_20F0,
                0x9F99_8C00,
            ]),
        );
    }

    #[test]
    fn borrowed_builder() {
        assert_eq!(
            debug::Data(
                EndpointNameBorrowed::builder(&mut Ump::random_buffer::<8>())
                    .name("Gimme some signal 🔊 🙌")
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0xF403_4769,
                0x6D6D_6520,
                0x736F_6D65,
                0x2073_6967,
                0xFC03_6E61,
                0x6C20_F09F,
                0x948A_20F0,
                0x9F99_8C00,
            ]),
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn name() {
        assert_eq!(
            EndpointNameMessage::from_data(&[
                0xF403_4769,
                0x6D6D_6520,
                0x736F_6D65,
                0x2073_6967,
                0xFC03_6E61,
                0x6C20_F09F,
                0x948A_20F0,
                0x9F99_8C00,
            ])
            .unwrap()
            .name(),
            std::string::String::from("Gimme some signal 🔊 🙌"),
        );
        assert_eq!(
            EndpointNameMessage::from_data(&[
                0xF403_4769,
                0x6D6D_6520,
                0x736F_6D65,
                0x206D_6F72,
                0xF803_6520,
                0x7369_676E,
                0x616C_20F0,
                0x9F94_8A20,
                0xFC03_F09F,
                0x998C_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .name(),
            std::string::String::from("Gimme some more signal 🔊 🙌"),
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn name_utf8_bytes() {
        assert_eq!(
            EndpointNameMessage::from_data(&[
                0xF403_4769,
                0x6D6D_6520,
                0x736F_6D65,
                0x2073_6967,
                0xFC03_6E61,
                0x6C20_F09F,
                0x948A_20F0,
                0x9F99_8C00,
            ])
            .unwrap()
            .name_utf8_bytes()
            .collect::<std::vec::Vec<u8>>(),
            std::vec![
                0x47, 0x69, 0x6D, 0x6D, 0x65, 0x20, 0x73, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x69, 0x67,
                0x6E, 0x61, 0x6C, 0x20, 0xF0, 0x9F, 0x94, 0x8A, 0x20, 0xF0, 0x9F, 0x99, 0x8C,
            ],
        );
    }
}
