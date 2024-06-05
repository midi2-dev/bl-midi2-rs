#![doc = include_str!("sysex7/README.md")]

use crate::{detail::common_err_strings, error::InvalidData};

#[derive(Eq, PartialEq, Clone, Debug, derive_more::From)]
pub enum Packet {
    ChannelVoice1(crate::channel_voice1::Packet),
    ChannelVoice2(crate::channel_voice2::Packet),
    FlexData(crate::flex_data::Packet),
    Sysex7(crate::sysex7::Packet),
    Sysex8(crate::sysex8::Packet),
    SystemCommon(crate::system_common::Packet),
    UmpStream(crate::ump_stream::Packet),
    Utility(crate::utility::Packet),
}

impl core::ops::Deref for Packet {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        match self {
            Self::ChannelVoice1(p) => p.deref(),
            Self::ChannelVoice2(p) => p.deref(),
            Self::FlexData(p) => p.deref(),
            Self::Sysex7(p) => p.deref(),
            Self::Sysex8(p) => p.deref(),
            Self::SystemCommon(p) => p.deref(),
            Self::UmpStream(p) => p.deref(),
            Self::Utility(p) => p.deref(),
        }
    }
}

impl<'a> core::convert::TryFrom<&'a [u32]> for Packet {
    type Error = crate::error::InvalidData;
    fn try_from(data: &'a [u32]) -> Result<Self, Self::Error> {
        use crate::{
            channel_voice1, channel_voice2, detail::BitOps, flex_data, sysex7, sysex8,
            system_common, ump_stream, utility,
        };

        if data.is_empty() {
            return Err(InvalidData(common_err_strings::ERR_SLICE_TOO_SHORT));
        }

        match u8::from(data[0].nibble(0)) {
            channel_voice1::UMP_MESSAGE_TYPE => Ok(channel_voice1::Packet::try_from(data)?.into()),
            channel_voice2::UMP_MESSAGE_TYPE => Ok(channel_voice2::Packet::try_from(data)?.into()),
            flex_data::UMP_MESSAGE_TYPE => Ok(flex_data::Packet::try_from(data)?.into()),
            sysex7::UMP_MESSAGE_TYPE => Ok(sysex7::Packet::try_from(data)?.into()),
            sysex8::UMP_MESSAGE_TYPE => Ok(sysex8::Packet::try_from(data)?.into()),
            system_common::UMP_MESSAGE_TYPE => Ok(system_common::Packet::try_from(data)?.into()),
            ump_stream::UMP_MESSAGE_TYPE => Ok(ump_stream::Packet::try_from(data)?.into()),
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
