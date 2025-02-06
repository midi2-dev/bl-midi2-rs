#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::Packets,
    midi2_proc::RebufferFrom,
    midi2_proc::TryRebufferFrom,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
)]
#[non_exhaustive]
pub enum UmpMessage<B: crate::buffer::Ump> {
    #[cfg(feature = "flex-data")]
    FlexData(crate::flex_data::FlexData<B>),
    #[cfg(feature = "channel-voice1")]
    ChannelVoice1(crate::channel_voice1::ChannelVoice1<B>),
    #[cfg(feature = "channel-voice2")]
    ChannelVoice2(crate::channel_voice2::ChannelVoice2<B>),
    #[cfg(feature = "sysex7")]
    Sysex7(crate::sysex7::Sysex7<B>),
    #[cfg(feature = "sysex8")]
    Sysex8(crate::sysex8::Sysex8<B>),
    #[cfg(feature = "system-common")]
    SystemCommon(crate::system_common::SystemCommon<B>),
    #[cfg(feature = "ump-stream")]
    UmpStream(crate::ump_stream::UmpStream<B>),
    #[cfg(feature = "utility")]
    Utility(crate::utility::Utility<B>),
}

impl<'a> core::convert::TryFrom<&'a [u32]> for UmpMessage<&'a [u32]> {
    type Error = crate::error::InvalidData;
    fn try_from(buffer: &'a [u32]) -> Result<Self, Self::Error> {
        use crate::detail::BitOps;
        use UmpMessage::*;

        if buffer.is_empty() {
            return Err(crate::error::InvalidData("Ump message slice is empty"));
        }

        Ok(match u8::from(buffer[0].nibble(0)) {
            #[cfg(feature = "flex-data")]
            crate::flex_data::UMP_MESSAGE_TYPE => {
                FlexData(crate::flex_data::FlexData::try_from(buffer)?)
            }
            #[cfg(feature = "channel-voice1")]
            crate::channel_voice1::UMP_MESSAGE_TYPE => {
                ChannelVoice1(crate::channel_voice1::ChannelVoice1::try_from(buffer)?)
            }
            #[cfg(feature = "channel-voice2")]
            crate::channel_voice2::UMP_MESSAGE_TYPE => {
                ChannelVoice2(crate::channel_voice2::ChannelVoice2::try_from(buffer)?)
            }
            #[cfg(feature = "sysex7")]
            crate::sysex7::UMP_MESSAGE_TYPE => Sysex7(crate::sysex7::Sysex7::try_from(buffer)?),
            #[cfg(feature = "sysex8")]
            crate::sysex8::UMP_MESSAGE_TYPE => Sysex8(crate::sysex8::Sysex8::try_from(buffer)?),
            #[cfg(feature = "system-common")]
            crate::system_common::UMP_MESSAGE_TYPE => {
                SystemCommon(crate::system_common::SystemCommon::try_from(buffer)?)
            }
            #[cfg(feature = "ump-stream")]
            crate::ump_stream::UMP_MESSAGE_TYPE => {
                UmpStream(crate::ump_stream::UmpStream::try_from(buffer)?)
            }
            #[cfg(feature = "utility")]
            crate::utility::UMP_MESSAGE_TYPE => Utility(crate::utility::Utility::try_from(buffer)?),
            _ => Err(crate::error::InvalidData(
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
    Copy,
    Debug,
    PartialEq,
    Eq,
)]
#[non_exhaustive]
#[cfg(any(
    feature = "channel-voice1",
    feature = "sysex7",
    feature = "system-common"
))]
pub enum BytesMessage<B: crate::buffer::Bytes> {
    #[cfg(feature = "channel-voice1")]
    ChannelVoice1(crate::channel_voice1::ChannelVoice1<B>),
    #[cfg(feature = "sysex7")]
    Sysex7(crate::sysex7::Sysex7<B>),
    #[cfg(feature = "system-common")]
    SystemCommon(crate::system_common::SystemCommon<B>),
}

#[cfg(any(
    feature = "channel-voice1",
    feature = "sysex7",
    feature = "system-common"
))]
impl<'a> core::convert::TryFrom<&'a [u8]> for BytesMessage<&'a [u8]> {
    type Error = crate::error::InvalidData;
    fn try_from(buffer: &'a [u8]) -> Result<Self, Self::Error> {
        if buffer.is_empty() {
            return Err(crate::error::InvalidData("Bytes slice is empty"));
        }
        use BytesMessage::*;

        Ok(match buffer[0] {
            #[cfg(feature = "channel-voice1")]
            0x80..=0xEF => ChannelVoice1(crate::channel_voice1::ChannelVoice1::try_from(buffer)?),
            #[cfg(feature = "sysex7")]
            0xF0 => Sysex7(crate::sysex7::Sysex7::try_from(buffer)?),
            #[cfg(feature = "system-common")]
            0xF1..=0xF6 | 0xF8..=0xFF => {
                SystemCommon(crate::system_common::SystemCommon::try_from(buffer)?)
            }
            _ => Err(crate::error::InvalidData(
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

    #[cfg(any(
        feature = "channel-voice1",
        feature = "sysex7",
        feature = "system-common"
    ))]
    static_assertions::assert_impl_all!(BytesMessage<&[u8]>: Copy);
    #[cfg(any(
        feature = "channel-voice1",
        feature = "sysex7",
        feature = "system-common"
    ))]
    static_assertions::assert_impl_all!(BytesMessage<[u8; 3]>: Copy);
    static_assertions::assert_impl_all!(UmpMessage<&[u32]>: Copy);
    static_assertions::assert_impl_all!(UmpMessage<[u32; 4]>: Copy);

    #[test]
    #[cfg(feature = "channel-voice1")]
    fn from_byte_data() {
        use crate::channel_voice1::ChannelVoice1;

        let buffer = [0xAB, 0x60, 0x33];
        let message = BytesMessage::try_from(&buffer[..]);
        let Ok(BytesMessage::ChannelVoice1(ChannelVoice1::KeyPressure(_))) = message else {
            panic!();
        };
    }

    #[cfg(feature = "channel-voice1")]
    #[test]
    fn rebuffer_slice_from_mut_slice_impl() {
        use crate::channel_voice1::ChannelPressure;
        use crate::Data;
        use crate::RebufferInto;

        let mut buffer = [0x0_u32, 0x0];

        let message_mut_slice: UmpMessage<&mut [u32]> =
            ChannelPressure::try_new_with_buffer(&mut buffer[..])
                .unwrap()
                .into();
        let message_slice: UmpMessage<&[u32]> = message_mut_slice.rebuffer_into();
        assert_eq!(message_slice.data(), &[0x20D0_0000_u32][..]);
    }

    #[cfg(feature = "ump-stream")]
    #[test]
    fn ump_stream() {
        use crate::ump_stream::UmpStream;

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

    #[cfg(feature = "sysex7")]
    #[test]
    fn packets() {
        use crate::Packets;

        let buffer = [
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
        let message = UmpMessage::try_from(&buffer[..]).unwrap();
        let mut packets = message.packets();

        assert_eq!(&*packets.next().unwrap(), &[0x3E16_0001, 0x0203_0405,][..]);
        assert_eq!(&*packets.next().unwrap(), &[0x3E26_0607, 0x0809_0A0B,][..]);
        assert_eq!(&*packets.next().unwrap(), &[0x3E26_0C0D, 0x0E0F_1011,][..]);
        assert_eq!(&*packets.next().unwrap(), &[0x3E26_1213, 0x1415_1617,][..]);
        assert_eq!(&*packets.next().unwrap(), &[0x3E26_1819, 0x1A1B_1C1D,][..]);
        assert_eq!(&*packets.next().unwrap(), &[0x3E26_1E1F, 0x2021_2223,][..]);
        assert_eq!(&*packets.next().unwrap(), &[0x3E26_2425, 0x2627_2829,][..]);
        assert_eq!(&*packets.next().unwrap(), &[0x3E26_2A2B, 0x2C2D_2E2F,][..]);
        assert_eq!(&*packets.next().unwrap(), &[0x3E32_3031, 0x0000_0000,][..]);
        assert_eq!(packets.next(), None);
    }

    #[cfg(feature = "flex-data")]
    #[test]
    fn flex_data() {
        use crate::flex_data::FlexData;

        let buffer = [0xD410_0105, 0x54C3_A172, 0x0, 0x0];
        let message = UmpMessage::try_from(&buffer[..]);
        let Ok(UmpMessage::FlexData(FlexData::ComposerName(_))) = message else {
            panic!();
        };
    }

    #[cfg(feature = "utility")]
    #[test]
    fn utility() {
        use crate::utility::Utility;

        let buffer = [0x0020_1234, 0x0, 0x0, 0x0];
        let message = UmpMessage::try_from(&buffer[..]);
        let Ok(UmpMessage::Utility(Utility::Timestamp(_))) = message else {
            panic!();
        };
    }

    #[cfg(feature = "channel-voice1")]
    #[test]
    fn from_level2() {
        use crate::channel_voice1::ChannelPressure;

        let level2_message = ChannelPressure::<[u32; 4]>::new();
        let _: UmpMessage<[u32; 4]> = level2_message.into();
    }

    #[cfg(feature = "flex-data")]
    #[test]
    fn from_level2_flex_data() {
        use crate::flex_data::Lyrics;

        let level2_message = Lyrics::<[u32; 4]>::new();
        let _: UmpMessage<[u32; 4]> = level2_message.into();
    }

    #[cfg(feature = "channel-voice1")]
    #[test]
    fn from_level2_bytes() {
        use crate::channel_voice1::ChannelPressure;

        let level2_message = ChannelPressure::<[u8; 3]>::new();
        let _: BytesMessage<[u8; 3]> = level2_message.into();
    }

    #[cfg(feature = "channel-voice2")]
    #[test]
    fn from_level2_channel_voice2() {
        use crate::channel_voice2::ChannelPressure;

        let level2_message = ChannelPressure::<[u32; 4]>::new();
        let _: UmpMessage<[u32; 4]> = level2_message.into();
    }

    #[cfg(feature = "ump-stream")]
    #[test]
    fn from_level2_ump_stream() {
        use crate::ump_stream::EndOfClip;

        let level2_message = EndOfClip::<[u32; 4]>::new();
        let _: UmpMessage<[u32; 4]> = level2_message.into();
    }

    #[cfg(feature = "utility")]
    #[test]
    fn from_level2_utility() {
        use crate::utility::DeltaClockstampTpq;

        let level2_message = DeltaClockstampTpq::<[u32; 4]>::new();
        let _: UmpMessage<[u32; 4]> = level2_message.into();
    }

    #[cfg(feature = "system-common")]
    #[test]
    fn from_level2_system_common() {
        use crate::system_common::Stop;

        let level2_message = Stop::<[u32; 4]>::new();
        let _: UmpMessage<[u32; 4]> = level2_message.into();
    }
}
