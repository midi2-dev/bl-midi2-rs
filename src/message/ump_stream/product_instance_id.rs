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

const STATUS: u16 = 0x4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductInstanceIdBorrowed<'a>(UmpStreamGroupBorrowed<'a>);

#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductInstanceIdOwned(UmpStreamGroupOwned);

#[derive(derive_more::From, Debug, Clone, PartialEq, Eq)]
pub enum ProductInstanceIdMessage<'a> {
    Borrowed(ProductInstanceIdBorrowed<'a>),
    #[cfg(feature = "std")]
    Owned(ProductInstanceIdOwned),
}

#[cfg(feature = "std")]
pub struct ProductInstanceIdBuilder<M: core::convert::From<ProductInstanceIdOwned>>(
    UmpStreamGroupBuilder<UmpStreamGroupOwned>,
    core::marker::PhantomData<M>,
);

pub struct ProductInstanceIdBorrowedBuilder<'a>(UmpStreamGroupBorrowedBuilder<'a>);

pub struct NameBytesIterator<'a>(
    core::iter::Filter<UmpStreamGroupPayloadIterator<'a>, fn(&u8) -> bool>,
);

pub trait ProductInstanceId: Data {
    #[cfg(feature = "std")]
    fn product_instance_id(
        &self,
    ) -> core::result::Result<std::string::String, std::string::FromUtf8Error> {
        std::string::String::from_utf8(self.product_instance_id_utf8_bytes().collect())
    }

    fn product_instance_id_utf8_bytes(&self) -> NameBytesIterator {
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
impl<'a> ProductInstanceIdMessage<'a> {
    pub fn builder() -> ProductInstanceIdBuilder<Self> {
        ProductInstanceIdBuilder::new()
    }
}

impl<'a> ProductInstanceIdBorrowed<'a> {
    pub fn builder(buffer: &'a mut [u32]) -> ProductInstanceIdBorrowedBuilder<'a> {
        ProductInstanceIdBorrowedBuilder::new(buffer)
    }
}

impl<'a> ProductInstanceId for ProductInstanceIdBorrowed<'a> {}
impl<'a> ProductInstanceId for ProductInstanceIdMessage<'a> {}
#[cfg(feature = "std")]
impl ProductInstanceId for ProductInstanceIdOwned {}

impl<'a> Data for ProductInstanceIdBorrowed<'a> {
    fn data(&self) -> &[u32] {
        self.0.data()
    }
}

#[cfg(feature = "std")]
impl Data for ProductInstanceIdOwned {
    fn data(&self) -> &[u32] {
        self.0.data()
    }
}

impl<'a> FromData<'a> for ProductInstanceIdBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Self(UmpStreamGroupBorrowed::from_data_unchecked(buffer))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        UmpStreamGroupBorrowed::validate_data(buffer)?;
        if super::status_from_buffer(buffer) != u10::new(STATUS) {
            return Err(Error::InvalidData);
        }
        Ok(())
    }
}

impl<'a> FromData<'a> for ProductInstanceIdMessage<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Self::Borrowed(ProductInstanceIdBorrowed::from_data_unchecked(buffer))
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        ProductInstanceIdBorrowed::validate_data(buffer)
    }
}

impl<'a> Data for ProductInstanceIdMessage<'a> {
    fn data(&self) -> &[u32] {
        use ProductInstanceIdMessage::*;
        match self {
            #[cfg(feature = "std")]
            Owned(m) => m.data(),
            Borrowed(m) => m.data(),
        }
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<ProductInstanceIdOwned>> core::default::Default
    for ProductInstanceIdBuilder<M>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl<M: core::convert::From<ProductInstanceIdOwned>> ProductInstanceIdBuilder<M> {
    pub fn new() -> Self {
        Self(
            UmpStreamGroupBuilder::new().status(u10::new(STATUS)),
            Default::default(),
        )
    }
    pub fn build(self) -> Result<M> {
        match self.0.build() {
            Ok(m) => Ok(ProductInstanceIdOwned(m).into()),
            Err(e) => Err(e),
        }
    }
    pub fn product_instance_id(mut self, product_instance_id_str: &str) -> Self {
        self.0 = self.0.payload(product_instance_id_str.bytes());
        self
    }
}

impl<'a> ProductInstanceIdBorrowedBuilder<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Self(UmpStreamGroupBorrowedBuilder::new(buffer).status(u10::new(STATUS)))
    }
    pub fn build(self) -> Result<ProductInstanceIdBorrowed<'a>> {
        match self.0.build() {
            Ok(m) => Ok(ProductInstanceIdBorrowed(m)),
            Err(e) => Err(e),
        }
    }
    pub fn product_instance_id(mut self, product_instance_id_str: &str) -> Self {
        self.0 = self.0.payload(product_instance_id_str.bytes());
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

    #[test]
    #[cfg(feature = "std")]
    fn builder() {
        assert_eq!(
            debug::Data(
                ProductInstanceIdMessage::builder()
                    .product_instance_id("🎹 PianoPulse ✨")
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0xF404_F09F,
                0x8EB9_2050,
                0x6961_6E6F,
                0x5075_6C73,
                0xFC04_6520,
                0xE29C_A800,
                0x0000_0000,
                0x0000_0000,
            ]),
        )
    }

    #[test]
    fn borrowed_builder() {
        assert_eq!(
            debug::Data(
                ProductInstanceIdBorrowed::builder(&mut Ump::random_buffer::<8>())
                    .product_instance_id("🎹 PianoPulse ✨")
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0xF404_F09F,
                0x8EB9_2050,
                0x6961_6E6F,
                0x5075_6C73,
                0xFC04_6520,
                0xE29C_A800,
                0x0000_0000,
                0x0000_0000,
            ]),
        )
    }

    #[test]
    #[cfg(feature = "std")]
    fn product_instance_id() {
        assert_eq!(
            ProductInstanceIdMessage::from_data(&[
                0xF404_F09F,
                0x8EB9_2050,
                0x6961_6E6F,
                0x5075_6C73,
                0xFC04_6520,
                0xE29C_A800,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .product_instance_id(),
            Ok(std::string::String::from("🎹 PianoPulse ✨")),
        )
    }
}
