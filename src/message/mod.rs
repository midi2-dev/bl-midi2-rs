use crate::{util::BitOps, *};

mod helpers;
// pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod system_common;
pub mod system_exclusive_7bit;
pub mod system_exclusive_8bit;
pub mod utility;

//pub use midi1_channel_voice::Midi1ChannelVoiceMessage;
pub use midi2_channel_voice::Midi2ChannelVoiceMessage;
pub use system_common::SystemCommonMessage;
pub use system_exclusive_7bit::Sysex7Message;
pub use system_exclusive_8bit::Sysex8Message;
pub use utility::UtilityMessage;

pub enum MidiMessage<'a, B: Buffer> {
    // Midi1ChannelVoice(Midi1ChannelVoiceMessage<'a>),
    Midi2ChannelVoice(Midi2ChannelVoiceMessage<'a>),
    Sysex7(Sysex7Message<'a, B>),
    Sysex8(Sysex8Message<'a>),
    Utility(UtilityMessage<'a>),
    SystemCommon(SystemCommonMessage<'a, B>),
}

use MidiMessage::*;

// const MIDI1_CHANNEL_VOICE_CODE: u8 = 2;
const MIDI2_CHANNEL_VOICE_CODE: u8 = 4;
const SYSEX7_CODE: u8 = 3;
const SYSEX8_CODE: u8 = 5;
const UTILITY_CODE: u8 = 0;
const SYSTEM_COMMON_CODE: u8 = 1;

impl<'a> Message<'a, Ump> for MidiMessage<'a, Ump> {
    fn data(&self) -> &'a [u32] {
        match self {
            // Midi1ChannelVoice(m) => m.data(),
            Midi2ChannelVoice(m) => m.data(),
            Sysex7(m) => m.data(),
            Sysex8(m) => m.data(),
            Utility(m) => m.data(),
            SystemCommon(m) => m.data(),
        }
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        match u8::from(buffer[0].nibble(0)) {
            // MIDI1_CHANNEL_VOICE_CODE => {
            //     Midi1ChannelVoice(Midi1ChannelVoiceMessage::from_data_unchecked(buffer))
            // }
            MIDI2_CHANNEL_VOICE_CODE => {
                Midi2ChannelVoice(Midi2ChannelVoiceMessage::from_data_unchecked(buffer))
            }
            SYSEX7_CODE => Sysex7(Sysex7Message::from_data_unchecked(buffer)),
            SYSEX8_CODE => Sysex8(Sysex8Message::from_data_unchecked(buffer)),
            UTILITY_CODE => Utility(UtilityMessage::from_data_unchecked(buffer)),
            SYSTEM_COMMON_CODE => SystemCommon(SystemCommonMessage::from_data_unchecked(buffer)),
            _ => panic!(),
        }
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        match u8::from(buffer[0].nibble(0)) {
            // MIDI1_CHANNEL_VOICE_CODE => Midi1ChannelVoiceMessage::validate_data(buffer),
            MIDI2_CHANNEL_VOICE_CODE => Midi2ChannelVoiceMessage::validate_data(buffer),
            SYSEX7_CODE => Sysex7Message::validate_data(buffer),
            SYSEX8_CODE => Sysex8Message::validate_data(buffer),
            UTILITY_CODE => UtilityMessage::validate_data(buffer),
            SYSTEM_COMMON_CODE => SystemCommonMessage::<Ump>::validate_data(buffer),
            _ => Err(Error::InvalidData),
        }
    }
}
