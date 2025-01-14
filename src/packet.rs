#![doc = include_str!("sysex7/README.md")]

use crate::{detail::common_err_strings, error::InvalidData};

#[derive(Eq, PartialEq, Clone, Debug, derive_more::From)]
pub enum Packet {
    #[cfg(feature = "channel-voice1")]
    ChannelVoice1(crate::channel_voice1::Packet),
    #[cfg(feature = "channel-voice2")]
    ChannelVoice2(crate::channel_voice2::Packet),
    #[cfg(feature = "flex-data")]
    FlexData(crate::flex_data::Packet),
    #[cfg(feature = "sysex7")]
    Sysex7(crate::sysex7::Packet),
    #[cfg(feature = "sysex8")]
    Sysex8(crate::sysex8::Packet),
    #[cfg(feature = "system-common")]
    SystemCommon(crate::system_common::Packet),
    #[cfg(feature = "ump-stream")]
    UmpStream(crate::ump_stream::Packet),
    #[cfg(feature = "utility")]
    Utility(crate::utility::Packet),
}

impl core::ops::Deref for Packet {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        match self {
            #[cfg(feature = "channel-voice1")]
            Self::ChannelVoice1(p) => p.deref(),
            #[cfg(feature = "channel-voice2")]
            Self::ChannelVoice2(p) => p.deref(),
            #[cfg(feature = "flex-data")]
            Self::FlexData(p) => p.deref(),
            #[cfg(feature = "sysex7")]
            Self::Sysex7(p) => p.deref(),
            #[cfg(feature = "sysex8")]
            Self::Sysex8(p) => p.deref(),
            #[cfg(feature = "system-common")]
            Self::SystemCommon(p) => p.deref(),
            #[cfg(feature = "ump-stream")]
            Self::UmpStream(p) => p.deref(),
            #[cfg(feature = "utility")]
            Self::Utility(p) => p.deref(),
        }
    }
}

impl<'a> core::convert::TryFrom<&'a [u32]> for Packet {
    type Error = crate::error::InvalidData;
    fn try_from(data: &'a [u32]) -> Result<Self, Self::Error> {
        #[cfg(feature = "channel-voice1")]
        use crate::channel_voice1;
        #[cfg(feature = "channel-voice2")]
        use crate::channel_voice2;
        use crate::detail::BitOps;
        #[cfg(feature = "flex-data")]
        use crate::flex_data;
        #[cfg(feature = "sysex7")]
        use crate::sysex7;
        #[cfg(feature = "sysex8")]
        use crate::sysex8;
        #[cfg(feature = "system-common")]
        use crate::system_common;
        #[cfg(feature = "ump-stream")]
        use crate::ump_stream;
        #[cfg(feature = "utility")]
        use crate::utility;

        if data.is_empty() {
            return Err(InvalidData(common_err_strings::ERR_SLICE_TOO_SHORT));
        }

        match u8::from(data[0].nibble(0)) {
            #[cfg(feature = "channel-voice1")]
            channel_voice1::UMP_MESSAGE_TYPE => Ok(channel_voice1::Packet::try_from(data)?.into()),
            #[cfg(feature = "channel-voice2")]
            channel_voice2::UMP_MESSAGE_TYPE => Ok(channel_voice2::Packet::try_from(data)?.into()),
            #[cfg(feature = "flex-data")]
            flex_data::UMP_MESSAGE_TYPE => Ok(flex_data::Packet::try_from(data)?.into()),
            #[cfg(feature = "sysex7")]
            sysex7::UMP_MESSAGE_TYPE => Ok(sysex7::Packet::try_from(data)?.into()),
            #[cfg(feature = "sysex8")]
            sysex8::UMP_MESSAGE_TYPE => Ok(sysex8::Packet::try_from(data)?.into()),
            #[cfg(feature = "system-common")]
            system_common::UMP_MESSAGE_TYPE => Ok(system_common::Packet::try_from(data)?.into()),
            #[cfg(feature = "ump-stream")]
            ump_stream::UMP_MESSAGE_TYPE => Ok(ump_stream::Packet::try_from(data)?.into()),
            #[cfg(feature = "utility")]
            utility::UMP_MESSAGE_TYPE => Ok(utility::Packet::try_from(data)?.into()),
            _ => Err(crate::error::InvalidData(
                common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Deref;

    #[test]
    #[cfg(feature = "utility")]
    fn construction() {
        let data = [0x0];
        assert_eq!(Packet::try_from(&data[..]).unwrap().deref(), &[0x0]);
    }

    #[test]
    fn construction_from_empty_data() {
        let data = [];
        assert_eq!(
            Packet::try_from(&data[..]),
            Err(InvalidData(common_err_strings::ERR_SLICE_TOO_SHORT))
        );
    }

    #[test]
    fn construction_from_reserved_ump_type_field() {
        let data = [0xE000_0000, 0x0, 0x0, 0x0];
        assert_eq!(
            Packet::try_from(&data[..]),
            Err(InvalidData(
                common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE
            ))
        );
    }
}
