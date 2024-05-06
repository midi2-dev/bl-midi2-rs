use crate::{channel_voice1, channel_voice2, flex_data, sysex7, sysex8, system_common, ump_stream};

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::JitterReduced,
    midi2_proc::RebufferFrom,
    midi2_proc::TryRebufferFrom,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[non_exhaustive]
pub enum UmpMessage<B: crate::buffer::Ump> {
    #[cfg(feature = "flex-data")]
    FlexData(flex_data::FlexData<B>),
    #[cfg(feature = "midi1-channel-voice")]
    ChannelVoice1(channel_voice1::ChannelVoice1<B>),
    #[cfg(feature = "midi2-channel-voice")]
    ChannelVoice2(channel_voice2::ChannelVoice2<B>),
    #[cfg(feature = "sysex7")]
    Sysex7(sysex7::Sysex7<B>),
    #[cfg(feature = "sysex8")]
    Sysex8(sysex8::Sysex8<B>),
    #[cfg(feature = "system-common")]
    SystemCommon(system_common::SystemCommon<B>),
    #[cfg(feature = "ump-stream")]
    UmpStream(ump_stream::UmpStream<B>),
}

impl<'a> core::convert::TryFrom<&'a [u32]> for UmpMessage<&'a [u32]> {
    type Error = crate::error::Error;
    fn try_from(buffer: &'a [u32]) -> Result<Self, Self::Error> {
        use crate::buffer::UmpPrivate;
        use crate::detail::BitOps;
        use UmpMessage::*;

        if buffer.message().len() < 1 {
            return Err(crate::error::Error::InvalidData(
                "Ump message slice is empty",
            ));
        }

        Ok(match u8::from(buffer.message()[0].nibble(0)) {
            #[cfg(feature = "flex-data")]
            flex_data::UMP_MESSAGE_TYPE => FlexData(flex_data::FlexData::try_from(buffer)?.into()),
            #[cfg(feature = "midi1-channel-voice")]
            channel_voice1::UMP_MESSAGE_TYPE => {
                ChannelVoice1(channel_voice1::ChannelVoice1::try_from(buffer)?.into())
            }
            #[cfg(feature = "midi2-channel-voice")]
            channel_voice2::UMP_MESSAGE_TYPE => {
                ChannelVoice2(channel_voice2::ChannelVoice2::try_from(buffer)?.into())
            }
            #[cfg(feature = "sysex7")]
            sysex7::UMP_MESSAGE_TYPE => Sysex7(sysex7::Sysex7::try_from(buffer)?.into()),
            #[cfg(feature = "sysex8")]
            sysex8::UMP_MESSAGE_TYPE => Sysex8(sysex8::Sysex8::try_from(buffer)?.into()),
            #[cfg(feature = "system-common")]
            system_common::UMP_MESSAGE_TYPE => {
                SystemCommon(system_common::SystemCommon::try_from(buffer)?.into())
            }
            #[cfg(feature = "ump-stream")]
            ump_stream::UMP_MESSAGE_TYPE => {
                UmpStream(ump_stream::UmpStream::try_from(buffer)?.into())
            }
            _ => Err(crate::error::Error::InvalidData(
                "Couldn't interpret ump message type",
            ))?,
        })
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
pub enum BytesMessage<B: crate::buffer::Bytes> {
    #[cfg(feature = "midi1-channel-voice")]
    ChannelVoice1(channel_voice1::ChannelVoice1<B>),
    #[cfg(feature = "sysex7")]
    Sysex7(sysex7::Sysex7<B>),
    #[cfg(feature = "system-common")]
    SystemCommon(system_common::SystemCommon<B>),
}

impl<'a> core::convert::TryFrom<&'a [u8]> for BytesMessage<&'a [u8]> {
    type Error = crate::error::Error;
    fn try_from(buffer: &'a [u8]) -> Result<Self, Self::Error> {
        if buffer.len() < 1 {
            return Err(crate::error::Error::InvalidData("Bytes slice is empty"));
        }
        use BytesMessage::*;

        Ok(match buffer[0] {
            0x80..=0xEF => ChannelVoice1(channel_voice1::ChannelVoice1::try_from(buffer)?.into()),
            0xF0 => Sysex7(sysex7::Sysex7::try_from(buffer)?.into()),
            0xF1..=0xF6 | 0xF8..=0xFF => {
                SystemCommon(system_common::SystemCommon::try_from(buffer)?.into())
            }
            _ => Err(crate::error::Error::InvalidData(
                "Couldn't interpret bytes message type",
            ))?,
        })
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::assert_eq;

    #[test]
    #[cfg(feature = "midi1-channel-voice")]
    fn from_byte_data() {
        use channel_voice1::ChannelVoice1;

        let buffer = [0xAB, 0x60, 0x33];
        let message = BytesMessage::try_from(&buffer[..]);
        let Ok(BytesMessage::ChannelVoice1(ChannelVoice1::KeyPressure(_))) = message else {
            panic!();
        };
    }

    #[cfg(feature = "ump-stream")]
    #[test]
    fn ump_stream() {
        use ump_stream::UmpStream;

        let buffer = [
            0xF412_0556,
            0x6962_7261,
            0x746F_5661,
            0x6E67_7561,
            0xF812_0572,
            0x643A_204C,
            0x6561_6469,
            0x6E67_2057,
            0xF812_0561,
            0x7665_7320,
            0x6F66_2045,
            0x7570_686F,
            0xF812_056E,
            0x79F0_9F9A,
            0x80F0_9F8E,
            0xB6F0_9F8C,
            0xFC12_058A,
            0x0000_0000,
            0x0000_0000,
            0x0000_0000,
        ];
        let message = UmpMessage::try_from(&buffer[..]);
        let Ok(UmpMessage::UmpStream(UmpStream::FunctionBlockName(_))) = message else {
            panic!();
        };
    }

    #[cfg(feature = "sysex8")]
    #[test]
    fn sysex8() {
        let buffer = [
            0x0020_1234,
            0x5E1E_BE00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5E2E_BE0D,
            0x0E0F_1011,
            0x1213_1415,
            0x1617_1819,
            0x5E2E_BE1A,
            0x1B1C_1D1E,
            0x1F20_2122,
            0x2324_2526,
            0x5E3C_BE27,
            0x2829_2A2B,
            0x2C2D_2E2F,
            0x3031_0000,
        ];
        let message = UmpMessage::try_from(&buffer[..]);
        let Ok(UmpMessage::Sysex8(_)) = message else {
            panic!();
        };
    }

    #[cfg(feature = "sysex7")]
    #[test]
    fn sysex7() {
        let buffer = [
            0x0000_0000,
            0x3E16_0001,
            0x0203_0405,
            0x3E26_0607,
            0x0809_0A0B,
            0x3E26_0C0D,
            0x0E0F_1011,
            0x3E26_1213,
            0x1415_1617,
            0x3E26_1819,
            0x1A1B_1C1D,
            0x3E26_1E1F,
            0x2021_2223,
            0x3E26_2425,
            0x2627_2829,
            0x3E26_2A2B,
            0x2C2D_2E2F,
            0x3E32_3031,
            0x0000_0000,
        ];
        let message = UmpMessage::try_from(&buffer[..]);
        let Ok(UmpMessage::Sysex7(_)) = message else {
            panic!();
        };
    }

    #[cfg(feature = "flex-data")]
    #[test]
    fn flex_data_builder() {
        use flex_data::FlexData;

        let buffer = [0x0, 0xD410_0105, 0x54C3_A172, 0x0, 0x0];
        let message = UmpMessage::try_from(&buffer[..]);
        let Ok(UmpMessage::FlexData(FlexData::ComposerName(_))) = message else {
            panic!();
        };
    }
}
