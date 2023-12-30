use crate::message::flex_data::flex_data_group::PayloadIterator;

impl<'a> core::iter::Iterator for TextIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct TextIterator<'a>(pub(crate) core::iter::Filter<PayloadIterator<'a>, fn(&u8) -> bool>);

#[cfg(not(feature = "std"))]
macro_rules! flex_data_text_message {
    ($root_ident:ident, $message_ident:ident, $borrowed_ident:ident, $borrowed_builder_ident:ident, $bank:expr, $status:expr) => {
        use crate::{
            message::flex_data::{
                bank_from_buffer, channel_from_buffer,
                flex_data_group::{
                    FlexDataGroup, FlexDataGroupBorrowed, FlexDataGroupBorrowedBuilder,
                },
                status_from_buffer,
                text::TextIterator,
                FlexData,
            },
            numeric_types::*,
            traits::{Data, FromData, Grouped, Level2Message},
            util::debug,
            Error, Result,
        };
        use midi2_attr::Data;

        #[derive(Clone, PartialEq, Eq)]
        pub struct $borrowed_ident<'a>(FlexDataGroupBorrowed<'a>);

        #[derive(derive_more::From, Clone, Data, Debug, PartialEq, Eq)]
        pub enum $message_ident<'a> {
            Borrowed($borrowed_ident<'a>),
        }

        pub struct $borrowed_builder_ident<'a, M: core::convert::From<$borrowed_ident<'a>>>(
            FlexDataGroupBorrowedBuilder<'a>,
            core::marker::PhantomData<M>,
        );

        pub trait $root_ident: Data {
            fn channel(&self) -> Option<u4> {
                channel_from_buffer(self.data())
            }
            fn text_bytes(&self) -> TextIterator {
                TextIterator(
                    FlexDataGroupBorrowed::from_data_unchecked(self.data())
                        .payload()
                        .filter(|v| *v != 0x0),
                )
            }
        }

        impl<'a> $borrowed_ident<'a> {
            pub fn builder(buffer: &'a mut [u32]) -> $borrowed_builder_ident<Self> {
                $borrowed_builder_ident::new(buffer)
            }
        }

        impl<'a> $root_ident for $borrowed_ident<'a> {}
        impl<'a> $root_ident for $message_ident<'a> {}

        impl<'a> FromData<'a> for $borrowed_ident<'a> {
            type Target = Self;
            fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
                $borrowed_ident(FlexDataGroupBorrowed::from_data_unchecked(buffer))
            }
            fn validate_data(buffer: &'a [u32]) -> Result<()> {
                let message = FlexDataGroupBorrowed::from_data(buffer)?;
                if status_from_buffer(message.data()) != $status {
                    return Err(Error::InvalidData);
                }
                if bank_from_buffer(message.data()) != $bank {
                    return Err(Error::InvalidData);
                }
                Ok(())
            }
        }

        impl<'a> FromData<'a> for $message_ident<'a> {
            type Target = Self;
            fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
                $message_ident::Borrowed($borrowed_ident::from_data_unchecked(buffer))
            }
            fn validate_data(buffer: &'a [u32]) -> Result<()> {
                $borrowed_ident::validate_data(buffer)
            }
        }

        impl<'a, M: core::convert::From<$borrowed_ident<'a>>> $borrowed_builder_ident<'a, M> {
            pub fn build(self) -> Result<M> {
                Ok($borrowed_ident(self.0.build()?).into())
            }
            pub fn text(mut self, string: &str) -> Self {
                self.0 = self.0.payload(string.bytes());
                self
            }
            pub fn group(mut self, group: u4) -> Self {
                self.0 = self.0.group(group);
                self
            }
            pub fn channel(mut self, channel: Option<u4>) -> Self {
                self.0 = self.0.channel(channel);
                self
            }
            pub fn new(buffer: &'a mut [u32]) -> Self {
                let mut builder = FlexDataGroupBorrowedBuilder::new(buffer);
                builder = builder.bank($bank);
                builder = builder.status($status);
                Self(builder, Default::default())
            }
        }

        impl<'a> Data for $borrowed_ident<'a> {
            fn data(&self) -> &[u32] {
                self.0.data()
            }
        }

        impl<'a> Grouped for $borrowed_ident<'a> {}
        impl<'a> Grouped for $message_ident<'a> {}

        impl<'a> FlexData for $borrowed_ident<'a> {}
        impl<'a> FlexData for $message_ident<'a> {}

        impl<'a> Level2Message for $borrowed_ident<'a> {}

        impl<'a> core::fmt::Debug for $borrowed_ident<'a> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                debug::packet_debug_fmt(self.0.data(), fmt)
            }
        }
    };
}

#[cfg(feature = "std")]
macro_rules! flex_data_text_message_std {
    ($root_ident:ident, $message_ident:ident, $borrowed_ident:ident, $owned_ident:ident, $builder_ident:ident, $borrowed_builder_ident:ident, $bank:expr, $status:expr) => {
        use crate::{
            message::flex_data::{
                bank_from_buffer, channel_from_buffer,
                flex_data_group::{
                    FlexDataGroup, FlexDataGroupBorrowed, FlexDataGroupBorrowedBuilder,
                    FlexDataGroupBuilder, FlexDataGroupOwned,
                },
                status_from_buffer,
                text::TextIterator,
                FlexData,
            },
            numeric_types::*,
            traits::{Data, FromData, Grouped, IntoOwned, Level2Message},
            util::debug,
            Error, Result,
        };
        use midi2_attr::Data;

        #[derive(Clone, PartialEq, Eq)]
        pub struct $owned_ident(FlexDataGroupOwned);

        #[derive(Clone, PartialEq, Eq)]
        pub struct $borrowed_ident<'a>(FlexDataGroupBorrowed<'a>);

        #[derive(derive_more::From, Clone, Data, Debug, PartialEq, Eq)]
        pub enum $message_ident<'a> {
            Borrowed($borrowed_ident<'a>),
            Owned($owned_ident),
        }
        pub struct $builder_ident<M: core::convert::From<$owned_ident>>(
            FlexDataGroupBuilder,
            core::marker::PhantomData<M>,
        );

        pub struct $borrowed_builder_ident<'a, M: core::convert::From<$borrowed_ident<'a>>>(
            FlexDataGroupBorrowedBuilder<'a>,
            core::marker::PhantomData<M>,
        );

        pub trait $root_ident: Data {
            fn channel(&self) -> Option<u4> {
                channel_from_buffer(self.data())
            }
            fn text(
                &self,
            ) -> core::result::Result<std::string::String, std::string::FromUtf8Error> {
                std::string::String::from_utf8(self.text_bytes().collect())
            }
            fn text_bytes(&self) -> TextIterator {
                TextIterator(
                    FlexDataGroupBorrowed::from_data_unchecked(self.data())
                        .payload()
                        .filter(|v| *v != 0x0),
                )
            }
        }

        impl<'a> $message_ident<'a> {
            pub fn builder() -> $builder_ident<Self> {
                $builder_ident::new()
            }
        }

        impl<'a> $borrowed_ident<'a> {
            pub fn builder(buffer: &'a mut [u32]) -> $borrowed_builder_ident<Self> {
                $borrowed_builder_ident::new(buffer)
            }
        }

        impl $root_ident for $owned_ident {}
        impl<'a> $root_ident for $borrowed_ident<'a> {}
        impl<'a> $root_ident for $message_ident<'a> {}

        impl<M: core::convert::From<$owned_ident>> $builder_ident<M> {
            pub fn build(&self) -> Result<M> {
                Ok($owned_ident(self.0.build()?).into())
            }
            pub fn text(&mut self, string: &str) -> &mut Self {
                self.0.payload(string.bytes());
                self
            }
            pub fn group(&mut self, group: u4) -> &mut Self {
                self.0.group(group);
                self
            }
            pub fn channel(&mut self, channel: Option<u4>) -> &mut Self {
                self.0.channel(channel);
                self
            }
            pub fn new() -> Self {
                let mut builder = FlexDataGroupBuilder::new();
                builder.bank($bank);
                builder.status($status);
                Self(builder, Default::default())
            }
        }

        impl<'a> FromData<'a> for $borrowed_ident<'a> {
            type Target = Self;
            fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
                $borrowed_ident(FlexDataGroupBorrowed::from_data_unchecked(buffer))
            }
            fn validate_data(buffer: &'a [u32]) -> Result<()> {
                let message = FlexDataGroupBorrowed::from_data(buffer)?;
                if status_from_buffer(message.data()) != $status {
                    return Err(Error::InvalidData);
                }
                if bank_from_buffer(message.data()) != $bank {
                    return Err(Error::InvalidData);
                }
                Ok(())
            }
        }

        impl<'a> FromData<'a> for $message_ident<'a> {
            type Target = Self;
            fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
                $message_ident::Borrowed($borrowed_ident::from_data_unchecked(buffer))
            }
            fn validate_data(buffer: &'a [u32]) -> Result<()> {
                $borrowed_ident::validate_data(buffer)
            }
        }

        impl<'a, M: core::convert::From<$borrowed_ident<'a>>> $borrowed_builder_ident<'a, M> {
            pub fn build(self) -> Result<M> {
                Ok($borrowed_ident(self.0.build()?).into())
            }
            pub fn text(mut self, string: &str) -> Self {
                self.0 = self.0.payload(string.bytes());
                self
            }
            pub fn group(mut self, group: u4) -> Self {
                self.0 = self.0.group(group);
                self
            }
            pub fn channel(mut self, channel: Option<u4>) -> Self {
                self.0 = self.0.channel(channel);
                self
            }
            pub fn new(buffer: &'a mut [u32]) -> Self {
                let mut builder = FlexDataGroupBorrowedBuilder::new(buffer);
                builder = builder.bank($bank);
                builder = builder.status($status);
                Self(builder, Default::default())
            }
        }

        impl<'a> Data for $borrowed_ident<'a> {
            fn data(&self) -> &[u32] {
                self.0.data()
            }
        }
        impl Data for $owned_ident {
            fn data(&self) -> &[u32] {
                self.0.data()
            }
        }

        impl<'a> Grouped for $borrowed_ident<'a> {}
        impl Grouped for $owned_ident {}
        impl<'a> Grouped for $message_ident<'a> {}

        impl<'a> FlexData for $borrowed_ident<'a> {}
        impl FlexData for $owned_ident {}
        impl<'a> FlexData for $message_ident<'a> {}

        impl<'a> IntoOwned for $borrowed_ident<'a> {
            type Owned = $owned_ident;
            fn into_owned(self) -> $owned_ident {
                $owned_ident(self.0.into_owned())
            }
        }
        impl<'a> IntoOwned for $message_ident<'a> {
            type Owned = $owned_ident;
            fn into_owned(self) -> $owned_ident {
                use $message_ident::*;
                match self {
                    Owned(m) => m,
                    Borrowed(m) => m.into_owned(),
                }
            }
        }

        impl<M: core::convert::From<$owned_ident>> core::default::Default for $builder_ident<M> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'a> Level2Message for $borrowed_ident<'a> {}
        impl Level2Message for $owned_ident {}

        impl<'a> core::fmt::Debug for $borrowed_ident<'a> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                debug::packet_debug_fmt(self.0.data(), fmt)
            }
        }
        impl core::fmt::Debug for $owned_ident {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                debug::packet_debug_fmt(self.0.data(), fmt)
            }
        }
    };
}

#[cfg(not(feature = "std"))]
pub(crate) use flex_data_text_message;
#[cfg(feature = "std")]
pub(crate) use flex_data_text_message_std;

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[cfg(not(feature = "std"))]
    flex_data_text_message!(
        Test,
        TestMessage,
        TestBorrowed,
        TestBorrowedBuilder,
        0xAC,
        0x3B
    );

    #[cfg(feature = "std")]
    flex_data_text_message_std!(
        Test,
        TestMessage,
        TestBorrowed,
        TestOwned,
        TestBuilder,
        TestBorrowedBuilder,
        0xAC,
        0x3B
    );

    #[test]
    #[cfg(feature = "std")]
    fn builder() {
        assert_eq!(
            debug::Data(TestMessage::builder()
                .text("Synth wizardry: turning knobs and flipping switches until it sounds like a laser battle in space! 💫🔊🚀")
                .group(u4::new(0xC))
                .channel(Some(u4::new(0xD)))
                .build()
                .unwrap()
                .data()),
            debug::Data(&[
                0xDC4D_AC3B,
                0x5379_6E74,
                0x6820_7769,
                0x7A61_7264,
                0xDC8D_AC3B,
                0x7279_3A20,
                0x7475_726E,
                0x696E_6720,
                0xDC8D_AC3B,
                0x6B6E_6F62,
                0x7320_616E,
                0x6420_666C,
                0xDC8D_AC3B,
                0x6970_7069,
                0x6E67_2073,
                0x7769_7463,
                0xDC8D_AC3B,
                0x6865_7320,
                0x756E_7469,
                0x6C20_6974,
                0xDC8D_AC3B,
                0x2073_6F75,
                0x6E64_7320,
                0x6C69_6B65,
                0xDC8D_AC3B,
                0x2061_206C,
                0x6173_6572,
                0x2062_6174,
                0xDC8D_AC3B,
                0x746C_6520,
                0x696E_2073,
                0x7061_6365,
                0xDC8D_AC3B,
                0x2120_F09F,
                0x92AB_F09F,
                0x948A_F09F,
                0xDCCD_AC3B,
                0x9A80_0000,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn borrowed_builder() {
        assert_eq!(
            debug::Data(TestBorrowed::builder(&mut [0x0; 40])
                .text("Synth wizardry: turning knobs and flipping switches until it sounds like a laser battle in space! 💫🔊🚀")
                .group(u4::new(0xC))
                .channel(Some(u4::new(0xD)))
                .build()
                .unwrap()
                .data()),
            debug::Data(&[
                0xDC4D_AC3B,
                0x5379_6E74,
                0x6820_7769,
                0x7A61_7264,
                0xDC8D_AC3B,
                0x7279_3A20,
                0x7475_726E,
                0x696E_6720,
                0xDC8D_AC3B,
                0x6B6E_6F62,
                0x7320_616E,
                0x6420_666C,
                0xDC8D_AC3B,
                0x6970_7069,
                0x6E67_2073,
                0x7769_7463,
                0xDC8D_AC3B,
                0x6865_7320,
                0x756E_7469,
                0x6C20_6974,
                0xDC8D_AC3B,
                0x2073_6F75,
                0x6E64_7320,
                0x6C69_6B65,
                0xDC8D_AC3B,
                0x2061_206C,
                0x6173_6572,
                0x2062_6174,
                0xDC8D_AC3B,
                0x746C_6520,
                0x696E_2073,
                0x7061_6365,
                0xDC8D_AC3B,
                0x2120_F09F,
                0x92AB_F09F,
                0x948A_F09F,
                0xDCCD_AC3B,
                0x9A80_0000,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            TestMessage::from_data(&[
                0xDC4D_AC3B,
                0x5379_6E74,
                0x6820_7769,
                0x7A61_7264,
                0xDC8D_AC3B,
                0x7279_3A20,
                0x7475_726E,
                0x696E_6720,
                0xDC8D_AC3B,
                0x6B6E_6F62,
                0x7320_616E,
                0x6420_666C,
                0xDC8D_AC3B,
                0x6970_7069,
                0x6E67_2073,
                0x7769_7463,
                0xDC8D_AC3B,
                0x6865_7320,
                0x756E_7469,
                0x6C20_6974,
                0xDC8D_AC3B,
                0x2073_6F75,
                0x6E64_7320,
                0x6C69_6B65,
                0xDC8D_AC3B,
                0x2061_206C,
                0x6173_6572,
                0x2062_6174,
                0xDC8D_AC3B,
                0x746C_6520,
                0x696E_2073,
                0x7061_6365,
                0xDC8D_AC3B,
                0x2120_F09F,
                0x92AB_F09F,
                0x948A_F09F,
                0xDCCD_AC3B,
                0x9A80_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .channel(),
            Some(u4::new(0xD)),
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn text() {
        assert_eq!(
            TestMessage::from_data(&[
                0xDC4D_AC3B,
                0x5379_6E74,
                0x6820_7769,
                0x7A61_7264,
                0xDC8D_AC3B,
                0x7279_3A20,
                0x7475_726E,
                0x696E_6720,
                0xDC8D_AC3B,
                0x6B6E_6F62,
                0x7320_616E,
                0x6420_666C,
                0xDC8D_AC3B,
                0x6970_7069,
                0x6E67_2073,
                0x7769_7463,
                0xDC8D_AC3B,
                0x6865_7320,
                0x756E_7469,
                0x6C20_6974,
                0xDC8D_AC3B,
                0x2073_6F75,
                0x6E64_7320,
                0x6C69_6B65,
                0xDC8D_AC3B,
                0x2061_206C,
                0x6173_6572,
                0x2062_6174,
                0xDC8D_AC3B,
                0x746C_6520,
                0x696E_2073,
                0x7061_6365,
                0xDC8D_AC3B,
                0x2120_F09F,
                0x92AB_F09F,
                0x948A_F09F,
                0xDCCD_AC3B,
                0x9A80_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .text()
            .unwrap(),
            "Synth wizardry: turning knobs and flipping switches until it sounds like a laser battle in space! 💫🔊🚀",
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn into_owned() {
        let _owned = {
            let data = [
                0xDC4D_AC3B,
                0x5379_6E74,
                0x6820_7769,
                0x7A61_7264,
                0xDC8D_AC3B,
                0x7279_3A20,
                0x7475_726E,
                0x696E_6720,
                0xDC8D_AC3B,
                0x6B6E_6F62,
                0x7320_616E,
                0x6420_666C,
                0xDC8D_AC3B,
                0x6970_7069,
                0x6E67_2073,
                0x7769_7463,
                0xDC8D_AC3B,
                0x6865_7320,
                0x756E_7469,
                0x6C20_6974,
                0xDC8D_AC3B,
                0x2073_6F75,
                0x6E64_7320,
                0x6C69_6B65,
                0xDC8D_AC3B,
                0x2061_206C,
                0x6173_6572,
                0x2062_6174,
                0xDC8D_AC3B,
                0x746C_6520,
                0x696E_2073,
                0x7061_6365,
                0xDC8D_AC3B,
                0x2120_F09F,
                0x92AB_F09F,
                0x948A_F09F,
                0xDCCD_AC3B,
                0x9A80_0000,
                0x0000_0000,
                0x0000_0000,
            ];
            let borrowed = TestMessage::from_data(&data).unwrap();
            borrowed.into_owned()
        };
    }

    #[test]
    fn status() {
        assert_eq!(
            TestMessage::from_data(&[
                0xDC4D_AC3B,
                0x5379_6E74,
                0x6820_7769,
                0x7A61_7264,
                0xDC8D_AC3B,
                0x7279_3A20,
                0x7475_726E,
                0x696E_6720,
                0xDC8D_AC3B,
                0x6B6E_6F62,
                0x7320_616E,
                0x6420_666C,
                0xDC8D_AC3B,
                0x6970_7069,
                0x6E67_2073,
                0x7769_7463,
                0xDC8D_AC3B,
                0x6865_7320,
                0x756E_7469,
                0x6C20_6974,
                0xDC8D_AC3B,
                0x2073_6F75,
                0x6E64_7320,
                0x6C69_6B65,
                0xDC8D_AC3B,
                0x2061_206C,
                0x6173_6572,
                0x2062_6174,
                0xDC8D_AC3B,
                0x746C_6520,
                0x696E_2073,
                0x7061_6365,
                0xDC8D_AC3B,
                0x2120_F09F,
                0x92AB_F09F,
                0x948A_F09F,
                0xDCCD_AC3B,
                0x9A80_0000,
                0x0000_0000,
                0x0000_0000,
            ])
            .unwrap()
            .status(),
            0x3B
        );
    }
}
